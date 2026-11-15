Showing code in an accessible way set me the challenges you will find on this page. Each one is a case below, written the way I write code in an article, and checked by the site's tests at every build.

## A line longer than the column

This is the most awkward case. The accessibility guidelines, at their highest level, ask for lines of at most 80 characters, but code cannot break just anywhere: code that breaks too often becomes unreadable. Besides, code easily holds rather long names. So by default I leave the block to scroll sideways, but the wrap toggle breaks the lines for a while, and switching it off brings the scroll back.

```rust
fn describe_the_whole_site_in_one_single_line_so_that_it_never_fits_the_text_column_of_any_screen(site: &SiteAddress, articles: &[Article]) -> String { format!("{site}{}", articles.len()) }
```

## A block that always fits

Always showing the wrap toggle would make no sense. Often the code fits the column even on the narrowest screen, as here, and then the toggle is not shown at all. A toggle that changes nothing would only be noise, for the eyes and for a screen reader.

```rust
let sum = 1 + 2;
println!("{sum}");
```

## Nested indentation

In code, indentation has a meaning, and in some languages, such as Python, it decides which lines belong together. Keeping its structure is essential. So when a line wraps, its continuation starts four columns deeper than the line itself, and reads as the same line, not as a new one at the same level.

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

The tab is very handy: one key, one character. Too bad its width is up to each of us: four columns in one editor, two in another, eight in a third. And the browser? If it drew tabs wider or narrower than the site counts them, the wrap toggle would show when it is not needed, or hide when it is, and a wrapped line would restart from the wrong indentation. So the site tells the browser how wide a tab is, four columns, and counts the same way.

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

Blank lines must be respected too. In ordinary text a web page drops them, but in code they separate one idea from the next, and whoever copies the code expects to find them. What you copy is the code as written, even with the wrap toggle on: no line breaks of its own, no word from the toggle.

```python
def load(path):
    with open(path) as handle:
        text = handle.read()

    return text.splitlines()
```
