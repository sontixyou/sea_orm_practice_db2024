## Setup

1. Install Rust
2. Install sea-orm-cli

```sh
cargo install sea-orm-cli
```

### Setup Database

```sh
docker compose build
docker compose up -d
```

### Migration

```sh
DATABASE_URL=mysql://root:password@localhost:3306/sea_orm_practice_db sea-orm-cli migrate refresh
```

## Description

main.rsには、以下のようなコードが書かれています。

- DBへレコードを作成、更新する関数
- 同一のDBを使用して、関数をテストする(mod testsと書いてあるテスト)
- テストケースごとにDBを作成して関数をテストする(mod tests_pallalelと書いてあるテスト)

test_helper.rsには、テストDB作成と削除のための関数を定義しています。
