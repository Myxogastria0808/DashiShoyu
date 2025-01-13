# だし醤油

## 構成

### Frontend

- Framework: Next.js 14
- CSS: css modules
- HTTP Client: fetch
- QR: next-qrcode
- Barcode: react-jsbarcode
- PDF: react-pdf

### Backend

- Web API: axum
- ORM: SeaORM
- Database: PostgreSQL
- Search Engine: MeiliSearch

### 画像専用の Backend

- Web API: Hono
- Image Storage: Cloudflare R2

#### Handler の命名規則

接尾辞にメソッドを明記すること

例: POST の handler の場合

```rust
async fn register_item_post() {
    //処理
}
```

## Setup

1. Nix のインストール

Nix をインストールしていない場合は、以下のコマンドでインストール

```sh
sh <(curl -L https://nixos.org/nix/install) --daemon
```

2. nix-shell

```sh
#dashishoyuディレクトリ
nix-shell
```

3. .env の作成

`backend/server/.env` を作成

4. docker の起動

```sh
#serverディレクトリ
docker-compose up -d
```

5. 初期データの投入等

```sh
bash init.sh
```

## エンティティを生成する

1. Docker の起動

```sh
#serverディレクトリ
docker-compose up -d
```

1. マイグレーションをする

```sh
cargo run --manifest-path ./migration/Cargo.toml -- refresh -u postgres://<POSTGRES_USER>:<POSTGRES_PASSWORD>@localhost:<POSTGRES_PORT>/<POSTGRES_DB>
```

コマンドの例

```sh
cargo run --manifest-path ./migration/Cargo.toml -- refresh -u postgres://username:password@localhost:5432/db_name
```

2. エンティティを生成する

```sh
#serverディレクトリ
rm entity
sea-orm-cli generate entity \
    -u <DATABASE_URL> \
    -o entity/src
```

コマンドの例

```sh
sea-orm-cli generate entity \
    -u postgres://username:password@localhost:5432/db_name \
    -o entity/src
```

## データベースに入る

```sh
docker exec -it postgrs psql -U <POSTGRES_USER> -d <POSTGRES_DB>
```

コマンドの例

```sh
docker exec -it postgrs psql -U username -d db_name
```

## server 環境の破壊

```sh
bash clean-up.sh
```

## DashiShoyu の ER 図

```mermaid
erDiagram
    Item ||--o| Label : "visible_id color"
    Item {
        i32 Id PK "autoincrement"
        String VisibleId UK, FK "Labelテーブルとリレーションを張っている"
        String Name
        String ProductNumber "型番 (わからない or 存在しない場合は、空の文字列)"
        String PhotoUrl UK "Cloudflare R2に画像を格納する ファイル名は、{Id}.webp"
        Record Record "enum {QR, Barcode, Nothing} (ActiveEnum)"
        String Description　"補足説明 (空の文字列を許容する)"
        Option_i32 YearPurchased "購入年度"
        Json Connector　"e.g. ['USB Type-C', 'USB Type-A']　(可変の配列)"
        datetime CreatedAt "登録したときの日時"
        datetime UpdatedAt "更新したときの日時"
    }
    Label {
        String VisibleId PK "実際の物品ID"
        Color Color "enum {Red, Orange, Brown, SkyBlue、Blue, Green, Yellow, Purple, Pink} (ActiveEnum)"
    }

    Object |o--|{ ObjectTagJunction : "tag"
    ObjectTagJunction }|--|| Tag : "tag"
    Object {
        i32 Id PK "autoincrement"
        String Name
        String PhotoUrl UK "Cloudflare R2に画像を格納する ファイル名は obj-{Id}.{各拡張子 MimeTypesから推測}"
        String MimeTypes
        String License
        String Description "補足説明 (空の文字列を許容する)"
        datetime CreatedAt "登録したときの日時"
        datetime UpdatedAt "更新したときの日時"
    }
    ObjectTagJunction {
        i32 Id PK "autoincrement"
        i32 ObjectId FK "Object Tableとリレーションを貼っている"
        i32 TagId FK "Tag Tableとリレーションを貼っている"
    }
    Tag {
        i32 Id PK "autoincrement"
        String name UK
    }
```

## 最終目標の ER 図

実際の物品 ID と物品そのものの ID を分離することで、QR が剥がれても問題ないようにしている

```mermaid
erDiagram
    Item ||--o{ Transaction : transaction
    Transaction }o--|{ User : user
    Item ||--o| Label : "visible_id color"
    Item {
        i32 Id PK "autoincrement"
        String VisibleId UK, FK "Labelテーブルとリレーションを張っている"
        String Name
        String ProductNumber "型番 (わからない or 存在しない場合は、空の文字列)"
        String PhotoUrl UK "Cloudflare R2に画像を格納する ファイル名は、{Id}.webp"
        Record Record "enum {QR, Barcode, Nothing} (ActiveEnum)"
        String Description　"補足説明 (空の文字列を許容する)"
        Option_i32 YearPurchased "購入年度"
        Json Connector　"e.g. ['USB Type-C', 'USB Type-A']　(可変の配列)"
        datetime CreatedAt "登録したときの日時"
        datetime UpdatedAt "更新したときの日時"
    }
    Label {
        String VisibleId PK "実際の物品ID"
        Color Color "enum {Red, Orange, Brown, SkyBlue、Blue, Green, Yellow, Purple, Pink} (ActiveEnum)"
    }
    Transaction {
        i32 Id PK, UK "autoincrement"
        USER borrow_user "USER table"
        USER agency_user "USER table, nullable"
        datetime borrow_date
        datetime due_date
        String state "enum{　'Pending', 'Lending', 'Completed', 'Delaying'　}でApp側でバリデーション"
    }
    User {
        i32 id PK, UK "autoincrement"
        String uid UK
    }

    Object |o--|{ ObjectTagJunction : "tag"
    ObjectTagJunction }|--|| Tag : "tag"
    Object {
        i32 Id PK "autoincrement"
        String Name
        String PhotoUrl UK "Cloudflare R2に画像を格納する ファイル名は obj-{Id}.{各拡張子 MimeTypesから推測}"
        String MimeTypes
        String License
        String Description "補足説明 (空の文字列を許容する)"
        datetime CreatedAt "登録したときの日時"
        datetime UpdatedAt "更新したときの日時"
    }
    ObjectTagJunction {
        i32 Id PK "autoincrement"
        i32 ObjectId FK "Object Tableとリレーションを貼っている"
        i32 TagId FK "Tag Tableとリレーションを貼っている"
    }
    Tag {
        i32 Id PK "autoincrement"
        String name UK
    }
```
