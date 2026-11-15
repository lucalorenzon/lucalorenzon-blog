Like code, tables are a critical point for a site meant to be responsive and accessible. My answer is a component of my own, which takes the table's place when the column is too narrow to give each of its columns a readable width. It always has two columns: on the left, one row for each field of the original table; on the right, the records, shown one at a time as they scroll past. A line above it says which record is shown, out of how many. Below are the cases I have dealt with, each written the way I write a table in an article, and checked by the site's tests at every build.

## A table that always fits

This is the simple case, the reference for the others: two short columns, shown as a table at every screen width. The tests check it at 767 and 768 pixels, the two sides of the width where the text grows and the column widens. A wider table, such as the next one, becomes a record viewer at 767.

| Tool | Use |
|---|---|
| Hammer | Nails |
| Saw | Wood |
| Drill | Holes |
| Level | Shelves |
| Ruler | Lengths |

## Six columns

At 768 pixels this table is shown as a table; at 767 it becomes the record viewer. The width where it changes form is estimated when the site is built, from the words in the table. This case also puts the keyboard to the test: the Tab key reaches the record viewer, and the arrow keys move through the records one at a time.

| Planet | Moons | Rings | Day | Year | Orbit |
|---|---|---|---|---|---|
| Mercury | None at all | None at all | 59 days | 88 days | Inner ring |
| Venus | None at all | None at all | 243 days | 225 days | Inner ring |
| Earth | One only | None at all | 24 hours | 365 days | Inner ring |
| Mars | Two small | None at all | 25 hours | 687 days | Inner ring |
| Jupiter | Very many | Faint ones | 10 hours | 12 years | Outer ring |

## Rows of different heights

The record viewer shows every record with each field at the same height. To make this possible, all the values of a field sit in the same place, hidden, so that its row is as tall as the tallest of them; long labels wrap, and so do the values. This way, moving to another record does not shift the labels. An empty value is shown as a dash. The tests check it at every width where this table is a record viewer.

| Name | Support level | Estimated time to the first answer of the support team, in working hours | Notes about the contract and its renewal conditions |
|---|---|---|---|
| Basic subscription | Email, in office hours only | 48 | Renewed every year unless one of the parties asks, in writing, to end it at least three months before the date it expires. |
| Plus subscription | Email | 24 | Renewed every month. |
| Premium subscription |  | 4 | Renewed every year, with a visit from the team at the start of each season and a report of everything that was done. |
| Custom subscription | Phone and email, day and night | 1 | Each clause is agreed with the customer. |

## A range with an en dash

Placeholder, to be replaced before the next version tag: write here what this case shows and why it matters: a range such as 63–44, which the browser would break after the en dash if the column were narrow, and which the site keeps on one line.

| Pts | What happened in the match |
|---|---|
| 63–44 | The home team led from the first minutes and never gave up the lead, whatever the visitors tried in the second half. |
| 71–69 | A close match decided in the last seconds by a shot from the other side of the court. |
| 58–57 | The visitors recovered from a deficit of fifteen points and lost only after an extra period. |
| 80–52 | A match without surprises, with the whole bench on the court well before the end. |
| 66–65 | Two teams that had never met before, with a very uncertain result until the last whistle. |

## A word with a hyphen

Placeholder, to be replaced before the next version tag: write here what this case shows and why it matters: a word such as read-only, which the browser would break after the hyphen if the column were narrow, and which the site keeps on one line.

| Mode | What it allows the reader of the file to do with the file |
|---|---|
| read-only | The reader opens the file and looks at it, but cannot change anything inside it, whatever the program they use for it. |
| write-once | The reader writes the file the first time it is created and never again, whatever the program they use for it. |
| read-write | The reader opens the file and changes whatever they like inside it, whatever the program they use for it. |
| append-only | The reader adds new lines at the end of the file and leaves the older lines as they were written. |

## Texts at the limit

Placeholder, to be replaced before the next version tag: write here what this case shows and why it matters: a table whose width the build estimates from the texts, written with italic, inline code and bold, so that it is a table at the width of a large phone, with very little room to spare.

| Plain | Italic | Code | Bold |
|---|---|---|---|
| Stationery | *Emphatically* | `notebook_cell_values` | **Importantly** |
| Pencil | *Lightly* | `pencil_ids` | **Strongly** |
| Eraser | *Gently* | `eraser_key` | **Firmly** |
| Ruler | *Precisely* | `ruler_size` | **Boldly** |
