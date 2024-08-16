mod emmet;

use crate::emmet::Node;

fn main() {
    let menu = Node::tag("nav")
        .attr("class", "menu")
        .attr("id", "main-menu")
        .child(
            Node::tag("ul")
                .child(Node::tag("li").child(Node::tag("a").attr("href", "/home").text("Home")))
                .child(
                    Node::tag("li").child(
                        Node::tag("a")
                            .attr("href", "/about")
                            .text("About")
                            .child(Node::tag("img")),
                    ),
                )
                .child(
                    Node::tag("li")
                        .child(Node::tag("h1").text("hello world").child(Node::tag("img")))
                        .child(Node::tag("a").attr("href", "/services").text("Services"))
                        .child(
                            Node::tag("ul")
                                .child(
                                    Node::tag("li").child(
                                        Node::tag("a")
                                            .attr("href", "/services/web-development")
                                            .text("Web Development"),
                                    ),
                                )
                                .child(
                                    Node::tag("li").child(
                                        Node::tag("a")
                                            .attr("href", "/services/app-development")
                                            .text("App Development"),
                                    ),
                                )
                                .child(Node::tag("li").child(
                                    Node::tag("a").attr("href", "/services/seo").text("SEO"),
                                )),
                        ),
                )
                .child(
                    Node::tag("li").child(Node::tag("a").attr("href", "/contact").text("Contact")),
                ),
        );

    println!("{menu}");
}
