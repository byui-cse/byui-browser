# CSE 199R — Browser Project Teams

Week 1 overview of the browser project teams and how their responsibilities fit together.

## Teams and Structure

This semester, CSE 199R and CSE 399R students will work together to build a browser from scratch. Each team owns a clear part of the browser stack and will coordinate with adjacent teams as features move from network response to parsed page, styled layout, script execution, and user interaction.

## Project Teams

1. The HTML Team
2. The CSS Engine Team
3. The Layout & Rendering Team
4. The JavaScript Engine Team
5. The JS APIs (Web APIs) Team
6. The Networking Team
7. The Security & Storage Team
8. The Browser UX Team
9. The Devtools Team

### The HTML Team

Builds the HTML parser that turns raw markup into the DOM tree, handling malformed and real-world HTML the way browsers actually have to. Owns the DOM API surface that other teams — CSS, JS, and Layout — will query and manipulate.

### The CSS Engine Team

Builds the CSS parser and the cascade, specificity, and inheritance resolution that turns stylesheets plus the DOM into a computed style for every element, also called the CSSOM. This team does not handle layout or painting; it determines what the styles are, not where things end up on screen.

### The Layout & Rendering Team

Takes the DOM and computed styles and builds the layout tree, computing box model, positioning, and text flow for every element. Then paints and composites the final pixels to the screen, including handling reflows when the page changes.

### The JavaScript Engine Team

Builds the core JS interpreter and engine: parsing, an execution model, and the runtime that executes scripts against the page. This is the lowest-level piece other JS-facing teams, including JS APIs and Devtools, will build on top of.

### The JS APIs (Web APIs) Team

Implements the browser-provided APIs scripts call, including DOM manipulation methods, fetch, timers, events, and related browser features. This team connects the JS Engine to the rest of the browser: DOM, Networking, and Storage.

### The Networking Team

Handles fetching resources over HTTP(S): requests, responses, redirects, caching, and connection management for pages, scripts, stylesheets, and other assets. Feeds parsed responses into the HTML, CSS, and JS pipelines.

### The Security & Storage Team

Implements same-origin policy, cookies, and client-side storage such as localStorage and IndexedDB, plus any permissions handling such as camera or geolocation. This team is responsible for making sure pages cannot do things they should not do to each other or to the user.

### The Browser UX Team

Builds the actual browser chrome: address bar, tabs, navigation controls, bookmarks, settings, and other user-facing controls. Wires that UI to the underlying engine — Networking, HTML, CSS, and Layout — to actually load and display pages.

### The Devtools Team

Builds the developer-facing inspection tools: DOM and style inspector, console, network panel, and any debugging tools for JS. Depends on hooks from nearly every other team, so this group will need close coordination across the whole project.
