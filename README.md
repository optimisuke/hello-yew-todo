# yew-todo

以下の記事を参考に Rust x Yew で todo アプリを作成

https://zenn.dev/azukiazusa/articles/rust-base-web-front-fremework-yew

https://github.com/azukiazusa1/yew-todo-app

# todo

- [x] 削除ボタンの追加
- [x] API 呼び出し

# point

- React と大体一緒
- api 呼び出しは、wasm の時は、reqwest だと対応してないものがあるので、gloo-net を使う
- Trunk をつかつと楽そう
- 所有権・借用周りで clone が増える
- TypeScript で良いんじゃねって気持ちはある
- CSS 周りのスタイルフレームワークがどうにかならないと流行らなさそう

# 使い方

## ビルド

```
trunk build --release
```

## 実行

```
trunk serve --port 3000
```

# 参考

https://crates.io/crates/gloo-net

https://docs.rs/gloo-net/latest/gloo_net/

https://yew.rs/ja/docs/0.18.0/getting-started/project-setup/using-trunk

https://yew.rs/ja/docs/next/more/css
