import express, { Response, Request } from "express";
import { dlopen, suffix } from "bun:ffi";

const app = express();
app.use(express.json());

const {
    symbols: { init, addQueue, fetchResult, fetchInputVec, fetchResultVec }
} = dlopen(`./libqueue.${suffix}`, {
    init: {},
    addQueue: { args: ["cstring"], returns: ["i64"] },
    fetchResult: { args: ["i64"], returns: ["cstring"] },
    fetchInputVec: { returns: ["cstring"] },
    fetchResultVec: { returns: ["cstring"] }
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

app.get("/fetchInputVec", (_req: Request, res: Response) => {
    return res.send(JSON.parse(fetchInputVec()));
});

app.get("/fetchResultVec", (_req: Request, res: Response) => {
    return res.send(JSON.parse(fetchResultVec()));
});

app.listen(12345);
