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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item "><a href="introduction.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item "><a href="tutorial/index.html"><strong aria-hidden="true">2.</strong> Tutorial</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="tutorial/create-zink-project.html"><strong aria-hidden="true">2.1.</strong> Creating Zink Project</a></li><li class="chapter-item "><a href="tutorial/compile-zink-project.html"><strong aria-hidden="true">2.2.</strong> Compiling Zink Project</a></li></ol></li><li class="chapter-item "><a href="examples/index.html"><strong aria-hidden="true">3.</strong> Examples</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="examples/add-two.html"><strong aria-hidden="true">3.1.</strong> AddTwo</a></li><li class="chapter-item "><a href="examples/fibonacci.html"><strong aria-hidden="true">3.2.</strong> Fibonacci</a></li><li class="chapter-item "><a href="examples/log.html"><strong aria-hidden="true">3.3.</strong> Log</a></li><li class="chapter-item "><a href="examples/select.html"><strong aria-hidden="true">3.4.</strong> Select</a></li><li class="chapter-item "><a href="examples/storage.html"><strong aria-hidden="true">3.5.</strong> Storage</a></li></ol></li><li class="chapter-item "><a href="styles/index.html"><strong aria-hidden="true">4.</strong> Styles</a></li><li class="chapter-item "><a href="compiler/index.html"><strong aria-hidden="true">5.</strong> Compiler</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="compiler/arithmetic.html"><strong aria-hidden="true">5.1.</strong> Arithmetic</a></li><li class="chapter-item "><a href="compiler/calls.html"><strong aria-hidden="true">5.2.</strong> Calls</a></li><li class="chapter-item "><a href="compiler/control-flow.html"><strong aria-hidden="true">5.3.</strong> Control Flow</a></li><li class="chapter-item "><a href="compiler/locals.html"><strong aria-hidden="true">5.4.</strong> Locals</a></li><li class="chapter-item "><a href="compiler/recursion.html"><strong aria-hidden="true">5.5.</strong> Recursion</a></li><li class="chapter-item "><a href="compiler/storage.html"><strong aria-hidden="true">5.6.</strong> Storage</a></li></ol></li><li class="chapter-item "><a href="stability/index.html"><strong aria-hidden="true">6.</strong> Stability</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="stability/v0.1.0.html"><strong aria-hidden="true">6.1.</strong> v0.1.0</a></li></ol></li><li class="chapter-item "><a href="security.html"><strong aria-hidden="true">7.</strong> Security</a></li><li class="chapter-item "><a href="benchmarks/index.html"><strong aria-hidden="true">8.</strong> Benchmarks</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="benchmarks/fibonacci.html"><strong aria-hidden="true">8.1.</strong> Fibonacci</a></li><li class="chapter-item "><a href="benchmarks/log.html"><strong aria-hidden="true">8.2.</strong> Log</a></li><li class="chapter-item "><a href="benchmarks/storage.html"><strong aria-hidden="true">8.3.</strong> Storage</a></li></ol></li><li class="chapter-item "><a href="cli/index.html"><strong aria-hidden="true">9.</strong> Command Line Tool</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="cli/elko.html"><strong aria-hidden="true">9.1.</strong> elko</a></li><li class="chapter-item "><a href="cli/zinkc.html"><strong aria-hidden="true">9.2.</strong> zinkc</a></li></ol></li><li class="chapter-item "><a href="contributing/index.html"><strong aria-hidden="true">10.</strong> Contributing</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="contributing/architecture.html"><strong aria-hidden="true">10.1.</strong> Architecture</a></li><li class="chapter-item "><a href="contributing/building.html"><strong aria-hidden="true">10.2.</strong> Building</a></li><li class="chapter-item "><a href="contributing/testing.html"><strong aria-hidden="true">10.3.</strong> Testing</a></li></ol></li><li class="chapter-item "><a href="appendix/index.html"><strong aria-hidden="true">11.</strong> Appendix</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="appendix/optimizations.html"><strong aria-hidden="true">11.1.</strong> A - Optimizations</a></li></ol></li></ol>';
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
