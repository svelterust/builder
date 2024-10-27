# builder

Imagine we had a page builder that was blazing fast for both the developer and the user.
One that makes editing pages instant without any lag or delay, while also providing blazing fast
SSR for the user when deployed. As a nice bonus, the project itself would be easy to deploy with
just a single static binary, built on modern standards.

To achieve this in this project, we'll use SvelteKit for the frontend, and Rust for the backend.
The key goal to making editing pages blazing fast is to make the element rendering run on the client-side, while
also sharing the same logic on the server-side. We achieve this by compiling the Rust code to WebAssembly, and using
it on the client-side to render the elements. For the server-side, Rust will render the elements natively
for blazing fast performance. To reduce the latency even further down to zero, we'll use libsql (a fork of SQLite)
to store all kinds of data in a single file/embedded replica, and making it easy to put the server close to the user.

To make the developer experience as good as possible, we'll also use SvelteKit with shadcn-svelte to build
an awesome UI for the page builder. The result will be a blazing fast page builder that's easy to use and
deploy.
