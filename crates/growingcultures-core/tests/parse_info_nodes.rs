use growingcultures_core::{Content, InfoNode};

#[test]
fn parse_text_node() {
    let s = r"---
title: Foo is bar 42
---

This is a paragraph.
";
    let node = InfoNode::try_from(("test", s)).unwrap();
    assert_eq!(
        node,
        InfoNode {
            ty: "test".into(),
            title: "Foo is bar 42".into(),
            items: vec![Content::Text("<p>This is a paragraph.</p>".into()),],
        }
    );
}

#[test]
fn parse_multiple_text_nodes() {
    let s = r"---
title: Foo is bar 42
---

This is a paragraph.

---

Another one.
";
    let node = InfoNode::try_from(("test", s)).unwrap();
    assert_eq!(
        node,
        InfoNode {
            ty: "test".into(),
            title: "Foo is bar 42".into(),
            items: vec![
                Content::Text("<p>This is a paragraph.</p>".into()),
                Content::Text("<p>Another one.</p>".into()),
            ],
        }
    );
}

#[test]
fn parse_image_node() {
    let s = r"---
title: Foo is bar 42
---

![Foo](https://example.com/foo.png)
";
    let node = InfoNode::try_from(("test", s)).unwrap();
    assert_eq!(
        node,
        InfoNode {
            ty: "test".into(),
            title: "Foo is bar 42".into(),
            items: vec![Content::Image(
                "https://example.com/foo.png".try_into().unwrap(),
                "Foo".into()
            ),],
        }
    );
}

#[test]
fn parse_audio_node() {
    let s = r"---
title: Foo is bar 42
---

[Foo](https://example.com/foo.mp3)
";
    let node = InfoNode::try_from(("test", s)).unwrap();
    assert_eq!(
        node,
        InfoNode {
            ty: "test".into(),
            title: "Foo is bar 42".into(),
            items: vec![Content::Audio(
                "https://example.com/foo.mp3".try_into().unwrap(),
                "Foo".into()
            ),],
        }
    );
}

#[test]
fn parse_video_node() {
    let s = r"---
title: Foo is bar 42
---

[Foo](https://example.com/foo.mp4)
";
    let node = InfoNode::try_from(("test", s)).unwrap();
    assert_eq!(
        node,
        InfoNode {
            ty: "test".into(),
            title: "Foo is bar 42".into(),
            items: vec![Content::Video(
                "https://example.com/foo.mp4".try_into().unwrap(),
                "Foo".into()
            ),],
        }
    );
}
