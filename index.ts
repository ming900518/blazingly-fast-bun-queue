import express, { Response, Request } from "express";
import { dlopen, suffix } from "bun:ffi";

const app = express();
app.use(express.json());

const {
    symbols: { init, addQueue, fetchResult }
} = dlopen(`./libqueue.${suffix}`, {
    init: {},
    addQueue: { args: ["cstring"], returns: ["i64"] },
    fetchResult: { args: ["i64"], returns: ["cstring"] }
});

init();

app.post("/addQueue", (req: Request, res: Response) => {
    const body = req.body;
    const queueId = Number(addQueue(Buffer.from(body.data, "utf8")));
    return res.send({ queueId });
});

app.get("/checkStatus/:queueId", (req: Request, res: Response) => {
    const queueId = req.params.queueId;
    const result = JSON.parse(fetchResult(Number(queueId)));
    return res.send(result);
});
app.listen(12345);
