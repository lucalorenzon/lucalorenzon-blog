Placeholder, to be replaced before the next version tag: write here what this page shows, in a few sentences: the code block, the component every fenced block of the Markdown becomes, and the cases below, each put to the test by the browser tests. Say that the blocks on this page are the real component, written the way an article's author writes them.

## A line longer than the column

Placeholder, to be replaced before the next version tag: write here what this case shows and why it matters: a line that does not fit the text column at any width, so the block scrolls sideways and offers the wrap toggle. Say what the reader sees with the toggle unchecked and checked.

```rust
fn describe_the_whole_site_in_one_single_line_so_that_it_never_fits_the_text_column_of_any_screen(site: &SiteAddress, articles: &[Article]) -> String { format!("{site}{}", articles.len()) }
```

## A block that always fits

Placeholder, to be replaced before the next version tag: write here what this case shows and why it matters: a short block that fits the text column even on the narrowest screen, so it never scrolls and never offers the wrap toggle.

```rust
let sum = 1 + 2;
println!("{sum}");
```

## Nested indentation

Placeholder, to be replaced before the next version tag: write here what this case shows and why it matters: lines at several indentation levels, made of spaces. A wrapped line continues four columns deeper than its own indentation, so the reader tells a continued line from a new one.

```rust
fn find_article(articles: &[Article], address: &str) -> Option<&Article> {
    for article in articles {
        if article.address == address {
            if article.is_published_on_the_site_and_visible_to_every_reader_without_asking {
                return Some(article);
            }
        }
    }
    None
}
```

## Indentation made of tabs

Placeholder, to be replaced before the next version tag: write here what this case shows and why it matters: lines indented with tab characters, which the browser draws four columns wide. The toggle and the wrapped indentation must agree with that width.

```rust
fn visit(node: &Node, depth: usize) {
	if node.is_leaf() {
		println!("leaf found at depth {depth} with the value {}", node.value());
		return;
	}
	for child in node.children() {
		visit(child, depth + 1);
	}
}
```

## A blank line inside a block

Placeholder, to be replaced before the next version tag: write here what this case shows and why it matters: an empty line in the middle of a block is kept, and copying the block gives it back as written, with no extra blank line.

```python
def load(path):
    with open(path) as handle:
        text = handle.read()

    return text.splitlines()
```
