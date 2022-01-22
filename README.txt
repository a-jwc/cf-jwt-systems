About:
This project is a simple social media web application with a simple authoritative server to serve JWT to the client for post creation authorization. The frontend was deployed with Cloudflare Pages and the backend was deployed with Workers. The posts are stored in Cloudflare KV, a key-value store deployed to a global edge network. 

Questions:
What new knowledge or skills did you take away from this project? If you learned a new language for this assignment, make sure to tell us.

While I had read about some of Cloudflare's products prior to this assignment, this was my first time using Cloudflare Workers, KV, Pages, and Tunnel. I thoroughly enjoyed working with KV and Wrangler, and I found them fairly simple and fast to get started. For this assigment, I learned and used Tailwind CSS for the first time, which I found quite fun. I had dabbled in Rust roughly two years ago through the rustlings course so I am also new to Rust. My experience with web development had been mostly with Node.js, but I found success using the Rocket web framework, which I had used a couple time in the recent past to build simple APIs similar to what was required for the Systems assignment. This was also my first time crafting my own JWT, which I found rewarding to accomplish.

What was the most difficult part of this assignment?

The most difficult part of the assignment was using Workers entirely in Rust, as the documentation was lacking due to it's fairly recent release. 

Did you attempt any extra credit? Tell us what you chose
