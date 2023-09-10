# Blazingly Fast Bun Queue
<img width="777" alt="F5pxioHbsAAj3Yq" src="https://github.com/ming900518/blazingly-fast-bun-queue/assets/15919723/41cd5c4b-907c-4ea0-97b2-73fa17b93107">

測試 Bun Express 後端 + Rust 多線程平行處理的佇列，利用 [Bun 的 FFI](https://bun.sh/docs/api/ffi) 實現 JavaScript 與 Rust 之間的溝通

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
**由於寫在 Rust 中的邏輯非常的簡單，所以可能沒辦法在性能比較強大的電腦中看到明顯的性能優勢，如果您會寫 Rust ，推薦先行在 Rust Library 中改寫 queue 的邏輯**
### 一、`/addQueue` 將資料加入佇列中等待處理
- Method： POST
- Resquest
```json
{
    "data": "Bun ❤️ Rust"
}
```
- Response
```json
{
    "queueId": 1694338428345
}
```
### 二、`/checkStatus/:queueId` 查詢佇列的處理結果
- Method： GET
- Resquest
將上一支 API 回傳的 queueId 作為 Path Variable 傳入即可
- Response
```json
{
    "processedCount": 1,
    "dataProcessTime": "2023-09-10 09:33:48.345724 UTC",
    "inputData": "Bun ❤️ Rust"
}
```
