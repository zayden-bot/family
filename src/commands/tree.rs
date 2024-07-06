use charming::{
    Chart,
    element::{Color, LineStyle},
    ImageFormat, ImageRenderer, series::{Graph, GraphData, GraphLayout, GraphLink, GraphNode, GraphNodeLabel},
};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateAttachment, CreateCommand,
    CreateCommandOption, EditInteractionResponse,
};

use crate::sqlx_lib::PostgresPool;

use super::{FamilyRow, Result};

#[derive(Debug)]
struct Node {
    pub id: i64,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub value: f64,
    pub category: u64,
    pub symbol_size: f64,
    pub link: Vec<i64>,
}

impl Node {
    fn new(id: i64, name: String, x: f64, y: f64) -> Self {
        Node {
            id,
            name,
            x,
            y,
            value: 0.0,
            category: 0,
            symbol_size: 100.0,
            link: Vec::new(),
        }
    }

    fn add_link(mut self, id: i64) -> Self {
        self.link.push(id);
        self
    }
}

impl From<&Node> for GraphNode {
    fn from(node: &Node) -> Self {
        GraphNode {
            id: node.id.to_string(),
            name: node.name.to_string(),
            x: node.x,
            y: node.y,
            value: node.value,
            category: node.category,
            symbol_size: node.symbol_size,
            label: Some(
                GraphNodeLabel::new()
                    .show(true)
                    .position("inside")
                    .formatter("{b}")
                    .color("white")
                    .font_size(22),
            ),
        }
    }
}

fn render_graph(data: GraphData) -> Result<Vec<u8>> {
    let chart = Chart::new().series(
        Graph::new()
            .layout(GraphLayout::None)
            .line_style(
                LineStyle::new()
                    .color(Color::Value(String::from("#ffffff")))
                    .width(10),
            )
            .data(data),
    );
    let mut renderer = ImageRenderer::new(1920, 1080);
    Ok(renderer.render_format(ImageFormat::Png, &chart)?)
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
    interaction.defer(ctx).await?;

    let pool = PostgresPool::get(ctx).await;

    let member = FamilyRow::safe_get(ctx, interaction.user.id).await?;

    let tree = member.tree(&pool, true, true).await?;

    let mut keys: Vec<i32> = tree.keys().copied().collect();
    keys.sort();

    let max_width = tree.values().map(|v| v.len()).max().unwrap_or(0);

    let mut nodes = Vec::new();
    for depth in keys {
        let values = tree.get(&depth).unwrap();
        let width = values.len();
        let width_diff = max_width - width;
        let spacing = width_diff as f64 / 2.0;
        for (index, value) in values.iter().enumerate() {
            let mut node = Node::new(
                value.id,
                value.username.clone(),
                spacing + index as f64,
                depth as f64,
            );
            for id in value.children_ids.iter().chain(value.partner_ids.iter()) {
                node = node.add_link(*id);
            }
            nodes.push(node);
        }
    }

    let data = GraphData {
        nodes: nodes.iter().map(GraphNode::from).collect(),
        links: nodes
            .iter()
            .flat_map(|node| {
                node.link.iter().map(|link| GraphLink {
                    source: node.id.to_string(),
                    target: link.to_string(),
                    value: None,
                })
            })
            .collect(),
        categories: Vec::new(),
    };

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new()
                .new_attachment(CreateAttachment::bytes(render_graph(data)?, "graph.png")),
        )
        .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("tree")
        .description("Adopt another user into your family.")
        .add_option(CreateCommandOption::new(
            CommandOptionType::User,
            "user",
            "The user to adopt.",
        ))
}
