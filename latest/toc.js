// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="intro/index.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="intro/quickstart.html"><strong aria-hidden="true">1.1.</strong> Quickstart</a></li><li class="chapter-item expanded "><a href="intro/commands.html"><strong aria-hidden="true">1.2.</strong> Commands</a></li><li class="chapter-item expanded "><a href="intro/options/index.html"><strong aria-hidden="true">1.3.</strong> Options</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="intro/options/global.html"><strong aria-hidden="true">1.3.1.</strong> Global</a></li><li class="chapter-item expanded "><a href="intro/options/render.html"><strong aria-hidden="true">1.3.2.</strong> Render</a></li><li class="chapter-item expanded "><a href="intro/options/export.html"><strong aria-hidden="true">1.3.3.</strong> Export</a></li><li class="chapter-item expanded "><a href="intro/options/backup.html"><strong aria-hidden="true">1.3.4.</strong> Backup</a></li><li class="chapter-item expanded "><a href="intro/options/filter.html"><strong aria-hidden="true">1.3.5.</strong> Filter</a></li><li class="chapter-item expanded "><a href="intro/options/preprocess.html"><strong aria-hidden="true">1.3.6.</strong> Pre-process</a></li><li class="chapter-item expanded "><a href="intro/options/postprocess.html"><strong aria-hidden="true">1.3.7.</strong> Post-process</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="templates/index.html"><strong aria-hidden="true">2.</strong> Templates</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="templates/an-example-template.html"><strong aria-hidden="true">2.1.</strong> An Example Template</a></li><li class="chapter-item expanded "><a href="templates/configuration/index.html"><strong aria-hidden="true">2.2.</strong> Configuration</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="templates/configuration/template-groups.html"><strong aria-hidden="true">2.2.1.</strong> Template Groups</a></li><li class="chapter-item expanded "><a href="templates/configuration/context-modes.html"><strong aria-hidden="true">2.2.2.</strong> Context Modes</a></li><li class="chapter-item expanded "><a href="templates/configuration/structure-modes.html"><strong aria-hidden="true">2.2.3.</strong> Structure Modes</a></li><li class="chapter-item expanded "><a href="templates/configuration/file-extensions.html"><strong aria-hidden="true">2.2.4.</strong> File Extensions</a></li><li class="chapter-item expanded "><a href="templates/configuration/names.html"><strong aria-hidden="true">2.2.5.</strong> Names</a></li></ol></li><li class="chapter-item expanded "><a href="templates/partial-templates.html"><strong aria-hidden="true">2.3.</strong> Partial Templates</a></li><li class="chapter-item expanded "><a href="templates/backlinks.html"><strong aria-hidden="true">2.4.</strong> Backlinks</a></li><li class="chapter-item expanded "><a href="templates/string-sanitization.html"><strong aria-hidden="true">2.5.</strong> String Sanitization</a></li><li class="chapter-item expanded "><a href="templates/context-reference/index.html"><strong aria-hidden="true">2.6.</strong> Context Reference</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="templates/context-reference/book.html"><strong aria-hidden="true">2.6.1.</strong> Book</a></li><li class="chapter-item expanded "><a href="templates/context-reference/annotation.html"><strong aria-hidden="true">2.6.2.</strong> Annotation</a></li><li class="chapter-item expanded "><a href="templates/context-reference/names.html"><strong aria-hidden="true">2.6.3.</strong> Names</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="apple-books/index.html"><strong aria-hidden="true">3.</strong> Apple Books</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="apple-books/macos/index.html"><strong aria-hidden="true">3.1.</strong> macOS</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="apple-books/macos/library-location.html"><strong aria-hidden="true">3.1.1.</strong> Library Location</a></li><li class="chapter-item expanded "><a href="apple-books/macos/archive-restore-library.html"><strong aria-hidden="true">3.1.2.</strong> Archive/Restore Library</a></li></ol></li><li class="chapter-item expanded "><a href="apple-books/ios/index.html"><strong aria-hidden="true">3.2.</strong> iOS</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="apple-books/ios/library-location.html"><strong aria-hidden="true">3.2.1.</strong> Library Location</a></li><li class="chapter-item expanded "><a href="apple-books/ios/access-library.html"><strong aria-hidden="true">3.2.2.</strong> Access Library</a></li><li class="chapter-item expanded "><a href="apple-books/ios/archive-restore-library.html"><strong aria-hidden="true">3.2.3.</strong> Archive/Restore Library</a></li></ol></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
