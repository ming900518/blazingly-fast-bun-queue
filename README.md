# Blazingly Fast Bun Queue

Bun **Express** 後端 + Rust 多線程平行處理佇列測試專案

利用 [Bun 的 FFI](https://bun.sh/docs/api/ffi) 實現 JavaScript 與 Rust 之間的溝通

[點我切換至 Hono 版（使用 Bun 的原生 Bun.serve() ，取得快三倍的性能提升！）](https://github.com/ming900518/blazingly-fast-bun-queue/tree/hono)

<img width="500" alt="F5pxioHbsAAj3Yq" src="https://github.com/ming900518/blazingly-fast-bun-queue/assets/15919723/41cd5c4b-907c-4ea0-97b2-73fa17b93107">

## 安裝

### 一、安裝必要依賴

1. [Bun](https://bun.sh)
2. [Rust](https://rustup.rs)
3. 在安裝的同時，順便把這個 Repository clone 下來

### 二、編譯 Rust Library

```shell
cd queue
cargo build --release
```

### 三、執行 Bun

```shell
cd .. # 回到 Repository 根目錄
bun run index.ts
```

## API

**由於 Rust Queue 的邏輯非常的簡單，所以可能沒辦法在性能比較強大的電腦中看到明顯的性能優勢，如果您會寫 Rust ，推薦先行改寫 Rust Library 中 queue 的邏輯後再進行測試**

### 一、`/addQueue` 將資料加入佇列中等待處理

-   Method： POST
-   Resquest

```json
{
    "data": "Bun ❤️ Rust"
}
```

-   Response

```json
{
    "queueId": 1694338428345
}
```

### 二、`/checkStatus/:queueId` 查詢佇列的處理結果

-   Method： GET
-   Resquest：將上一支 API 回傳的 queueId 作為 Path Variable 傳入即可
-   Response

```json
{
    "processedCount": 1,
    "dataProcessTime": "2023-09-10 09:33:48.345724 UTC",
    "inputData": "Bun ❤️ Rust"
}
```

### 三、`/fetchInputVec` 查看存放在 Rust Library 中的佇列等待處理清單

> 註：我寫在 Rust Library 中的邏輯太簡單了， Bun 的單線程處理速度也沒有 Rust 的多線程快，所以如果不改程式碼，基本上只看得到空陣列 😂

-   Method： GET
-   Resquest：無
-   Response

```json
[
    [1694340796002, "Bun ❤️ Rust"],
    [1694340821274, "Bun ❤️ Rust"]
]
```

### 四、`/fetchResultVec` 查看存放在 Rust Library 中的佇列處理結果清單

-   Method： GET
-   Resquest：無
-   Response

```json
[
    [1694340796002, "2023-09-10 10:13:16.002754 UTC", "Bun ❤️ Rust"],
    [1694340821274, "2023-09-10 10:13:41.274656 UTC", "Bun ❤️ Rust"]
]
```

## Benchmark

-   設備：MacBook Air M1 (2020), 16GB RAM
-   OS：macOS 14.0 Sonoma
-   指令：`hey -z 30s -c 100 -m POST -D test.json -T "application/json" http://localhost:12345/addQueue`

```
Summary:
  Total:	30.0076 secs
  Slowest:	0.0514 secs
  Fastest:	0.0044 secs
  Average:	0.0063 secs
  Requests/sec:	15936.3293

  Total data:	11955275 bytes
  Size/request:	25 bytes

Response time histogram:
  0.004 [1]	|
  0.009 [454809]	|■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.014 [22256]	|■■
  0.019 [999]	|
  0.023 [23]	|
  0.028 [26]	|
  0.033 [12]	|
  0.037 [15]	|
  0.042 [10]	|
  0.047 [27]	|
  0.051 [33]	|


Latency distribution:
  10% in 0.0051 secs
  25% in 0.0052 secs
  50% in 0.0054 secs
  75% in 0.0072 secs
  90% in 0.0084 secs
  95% in 0.0091 secs
  99% in 0.0108 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0000 secs, 0.0044 secs, 0.0514 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0040 secs
  req write:	0.0000 secs, 0.0000 secs, 0.0006 secs
  resp wait:	0.0063 secs, 0.0044 secs, 0.0438 secs
  resp read:	0.0000 secs, 0.0000 secs, 0.0005 secs

Status code distribution:
  [200]	478211 responses
```
