This is the frame of every page of the site: it makes sure each page has a header, a footer and its content in the middle. For now the frame is a single component. The site is still a draft and shows no particular need to split it into smaller ones, so I left it whole rather than divide it too early.

Since it is the framework the pages stand on, the frame carries the responsive behaviour of the page's layout: change the width of the window and you will see the site change its look. I tried to follow what Jen Simmons calls [intrinsic web design](https://zeldman.com/2018/05/02/transcript-intrinsic-web-design-with-jen-simmons-the-big-web-show/): a layout that mixes fixed, fluid and content-sized pieces, each adapting to the room it has. The text follows [fluid typography](https://www.smashingmagazine.com/2023/11/addressing-accessibility-concerns-fluid-type/): its size grows continuously within a fixed range.

## The header

For the logo I chose the photo I now use everywhere to represent myself: my wife took it while I was pushing my twin daughters in the summer heat. I jokingly call it *Il peso della prole*, "the weight of offspring", but it is not a negative weight: it is simply my condition as a father, where the effort cannot be separated from the joy of having them.

By chance I noticed that, by lowering the line a little, I could make it run under the stroller's wheels, like the ground it rolls on. Beside the photo I then put my nickname, Lvk@73r. The name sitting on the line did not look good, so I broke the line, as if the name were a piece of it.

On the right, the header keeps some room free for a menu that does not exist yet. Below 1280 pixels of width, the header has two rows: the logo above the line, the page's title below it. From 1280 pixels up, there is room for one row, and the title moves between the logo and the menu's room. This is the only change of the frame at a fixed width.

## The text grows with the page

Between a page 320 pixels wide and one 1366 pixels wide, each of these sizes grows on a straight line, and it stays the same beyond:

- the column's text, from 18 to 20 pixels;
- the page's title, from 24 to 32 pixels;
- the logo, from 48 to 64 pixels;
- the footer's text, from 16 to 18 pixels.

### The headings grow with the text

The headings follow the column's text instead of a range of their own: a section heading, like the one above the list, is always 1.2 times the column's text, and a sub-section heading, like this one, 1.1 times.

## The column

The text is set in a single column, centred, at most 60 characters wide. It is the content-sized piece of the layout: on a narrow screen the column fills the width; on a wide one it stops at its limit, and since its characters grow with the page, so does the column. Everything the text holds, headings, lists, code and tables, lives inside this column; how each of them looks is on its own page of this style guide.

Below the column, the footer stays at the bottom of the window while you scroll. It holds the copyright and the terms on which you may reuse the page, as a link to the page about me, which explains them.
