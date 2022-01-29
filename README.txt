About:
This project is a simple social media web application with a simple authentication server to serve a JWT to authenticate a user prior to post creation. The frontend was deployed with Cloudflare Pages and the backend was deployed with Workers. The posts are stored in Cloudflare KV, a key-value store deployed to a global edge network. This project features "flash accounts", where a username would be valid for 24 hours before expiring, forever locking the username from use by anyone.

Questions:
1. What new knowledge or skills did you take away from this project? If you learned a new language for this assignment, make sure to tell us.

While I had read about some of Cloudflare's products prior to this assignment, this was my first time using Cloudflare Workers, KV, Pages, and Tunnel. I thoroughly enjoyed working with KV and Wrangler, and I found them fairly simple and fast to get started. For this assigment, I learned and used Tailwind CSS for the first time, which I found quite fun. I had dabbled in Rust roughly two years ago through the rustlings course so I am also new to Rust. My experience with web development had been mostly with Node.js, but I found success using the Rocket web framework, which I had used a couple time in the recent past to build APIs . This was also my first time crafting my own JWT, which I found rewarding to accomplish.

2. What was the most difficult part of this assignment?

The most difficult part of the assignment was integrating the two parts of the assignment together. Another difficulty I had encountered was with the limited amount of online documentation and tutorials for workers-rs, although I was able to overcome much of the hardships through experimentation. Even though I am appreciative of the Systems Assignment Grader, I found that working with the limited feedback provided by the Systems Assignment Grader was also challenging, particularly the extra credit section. 

3. Did you attempt any extra credit? Tell us what you chose

I had attempted several extra credit options:
General Assignment
- User posts (I had added a small post form for users to input a username, title, content, and optional image/gif file)
- Content Variety (I had added the option for users to upload an image/gif)
- Interactivity (I had implemented a voting system similarly to Medium's)

Systems Assignment
- README.txt end point (This is what you are reading right now and is properly served from the requested GET endpoint!)
- Integration with General Assignment (I had attempted to integrate the authentication service as directed by the online assingment documentation, but was unable to pass the Systems Assignment Grader for this extra credit portion. As this testing service is a blackbox, I am unsure where I am failing.)
