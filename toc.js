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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="intro/main.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="intro/overview.html"><strong aria-hidden="true">1.1.</strong> Overview</a></li><li class="chapter-item expanded "><a href="intro/usage.html"><strong aria-hidden="true">1.2.</strong> Usage</a></li><li class="chapter-item expanded "><a href="intro/performance.html"><strong aria-hidden="true">1.3.</strong> Performance</a></li></ol></li><li class="chapter-item expanded "><a href="tools/main.html"><strong aria-hidden="true">2.</strong> Development tooling</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tools/debugger.html"><strong aria-hidden="true">2.1.</strong> Debugger</a></li><li class="chapter-item expanded "><a href="tools/repl.html"><strong aria-hidden="true">2.2.</strong> REPL</a></li></ol></li><li class="chapter-item expanded "><a href="user_docs/main.html"><strong aria-hidden="true">3.</strong> User Documentation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="user_docs/assembly/main.html"><strong aria-hidden="true">3.1.</strong> Miden Assembly</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="user_docs/assembly/code_organization.html"><strong aria-hidden="true">3.1.1.</strong> Code Organization</a></li><li class="chapter-item expanded "><a href="user_docs/assembly/execution_contexts.html"><strong aria-hidden="true">3.1.2.</strong> Execution contexts</a></li><li class="chapter-item expanded "><a href="user_docs/assembly/flow_control.html"><strong aria-hidden="true">3.1.3.</strong> Flow Control</a></li><li class="chapter-item expanded "><a href="user_docs/assembly/field_operations.html"><strong aria-hidden="true">3.1.4.</strong> Field Operations</a></li><li class="chapter-item expanded "><a href="user_docs/assembly/u32_operations.html"><strong aria-hidden="true">3.1.5.</strong> u32 Operations</a></li><li class="chapter-item expanded "><a href="user_docs/assembly/stack_manipulation.html"><strong aria-hidden="true">3.1.6.</strong> Stack manipulation</a></li><li class="chapter-item expanded "><a href="user_docs/assembly/io_operations.html"><strong aria-hidden="true">3.1.7.</strong> Input / Output Operations</a></li><li class="chapter-item expanded "><a href="user_docs/assembly/cryptographic_operations.html"><strong aria-hidden="true">3.1.8.</strong> Cryptographic Operations</a></li><li class="chapter-item expanded "><a href="user_docs/assembly/events.html"><strong aria-hidden="true">3.1.9.</strong> Events</a></li><li class="chapter-item expanded "><a href="user_docs/assembly/debugging.html"><strong aria-hidden="true">3.1.10.</strong> Debugging</a></li></ol></li><li class="chapter-item expanded "><a href="user_docs/stdlib/main.html"><strong aria-hidden="true">3.2.</strong> Miden Standard Library</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="user_docs/stdlib/collections.html"><strong aria-hidden="true">3.2.1.</strong> std::collections</a></li><li class="chapter-item expanded "><a href="user_docs/stdlib/crypto/dsa.html"><strong aria-hidden="true">3.2.2.</strong> std::crypto::dsa</a></li><li class="chapter-item expanded "><a href="user_docs/stdlib/crypto/fri.html"><strong aria-hidden="true">3.2.3.</strong> std::crypto::fri</a></li><li class="chapter-item expanded "><a href="user_docs/stdlib/crypto/hashes.html"><strong aria-hidden="true">3.2.4.</strong> std::crypto::hashes</a></li><li class="chapter-item expanded "><a href="user_docs/stdlib/math/u64.html"><strong aria-hidden="true">3.2.5.</strong> std::math::u64</a></li><li class="chapter-item expanded "><a href="user_docs/stdlib/mem.html"><strong aria-hidden="true">3.2.6.</strong> std::mem</a></li><li class="chapter-item expanded "><a href="user_docs/stdlib/sys.html"><strong aria-hidden="true">3.2.7.</strong> std:sys</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="design/main.html"><strong aria-hidden="true">4.</strong> Design</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="design/programs.html"><strong aria-hidden="true">4.1.</strong> Programs</a></li><li class="chapter-item expanded "><a href="design/decoder/main.html"><strong aria-hidden="true">4.2.</strong> Program decoder</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="design/decoder/constraints.html"><strong aria-hidden="true">4.2.1.</strong> Decoder constraints</a></li></ol></li><li class="chapter-item expanded "><a href="design/stack/main.html"><strong aria-hidden="true">4.3.</strong> Operand stack</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="design/stack/op_constraints.html"><strong aria-hidden="true">4.3.1.</strong> Operation constraints</a></li><li class="chapter-item expanded "><a href="design/stack/system_ops.html"><strong aria-hidden="true">4.3.2.</strong> System operations</a></li><li class="chapter-item expanded "><a href="design/stack/field_ops.html"><strong aria-hidden="true">4.3.3.</strong> Field operations</a></li><li class="chapter-item expanded "><a href="design/stack/u32_ops.html"><strong aria-hidden="true">4.3.4.</strong> u32 operations</a></li><li class="chapter-item expanded "><a href="design/stack/stack_ops.html"><strong aria-hidden="true">4.3.5.</strong> Stack manipulation</a></li><li class="chapter-item expanded "><a href="design/stack/io_ops.html"><strong aria-hidden="true">4.3.6.</strong> Input / output operations</a></li><li class="chapter-item expanded "><a href="design/stack/crypto_ops.html"><strong aria-hidden="true">4.3.7.</strong> Cryptographic operations</a></li></ol></li><li class="chapter-item expanded "><a href="design/range.html"><strong aria-hidden="true">4.4.</strong> Range Checker</a></li><li class="chapter-item expanded "><a href="design/chiplets/main.html"><strong aria-hidden="true">4.5.</strong> Chiplets</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="design/chiplets/hasher.html"><strong aria-hidden="true">4.5.1.</strong> Hash Chiplet</a></li><li class="chapter-item expanded "><a href="design/chiplets/bitwise.html"><strong aria-hidden="true">4.5.2.</strong> Bitwise Chiplet</a></li><li class="chapter-item expanded "><a href="design/chiplets/memory.html"><strong aria-hidden="true">4.5.3.</strong> Memory Chiplet</a></li><li class="chapter-item expanded "><a href="design/chiplets/kernel_rom.html"><strong aria-hidden="true">4.5.4.</strong> Kernel ROM Chiplet</a></li></ol></li><li class="chapter-item expanded "><a href="design/lookups/main.html"><strong aria-hidden="true">4.6.</strong> Lookup arguments</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="design/lookups/multiset.html"><strong aria-hidden="true">4.6.1.</strong> Multiset checks</a></li><li class="chapter-item expanded "><a href="design/lookups/logup.html"><strong aria-hidden="true">4.6.2.</strong> LogUp</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="background.html"><strong aria-hidden="true">5.</strong> Background Material</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
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
