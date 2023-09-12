import { dlopen, suffix } from "bun:ffi";
import { Hono } from "hono";

const app = new Hono();

const {
    symbols: { init, addQueue, fetchResult, fetchInputVec, fetchResultVec }
} = dlopen(`./queue/target/release/libqueue.${suffix}`, {
    init: {},
    addQueue: { args: ["cstring"], returns: ["i64"] },
    fetchResult: { args: ["i64"], returns: ["cstring"] },
    fetchInputVec: { returns: ["cstring"] },
    fetchResultVec: { returns: ["cstring"] }
});

init();

app.post("/addQueue", async (c) => {
    const body = await c.req.json();
    const queueId = Number(addQueue(Buffer.from(body.data, "utf8")));
    return c.json({ queueId });
});

app.get("/checkStatus/:queueId", (c) => {
    const queueId = c.req.param("queueId");
    const result = JSON.parse(fetchResult(Number(queueId)));
    return c.json(result);
});

app.get("/fetchInputVec", (c) => {
    return c.json(JSON.parse(fetchInputVec()));
});

app.get("/fetchResultVec", (c) => {
    return c.json(JSON.parse(fetchResultVec()));
});

export default {
    port: 12345,
    fetch: app.fetch
};
