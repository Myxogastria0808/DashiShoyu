# だし醤油

## 構成

### Frontend

- Framework: Next.js 14
- CSS: css modules
- HTTP Client: ky
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

`server/.env` を作成

`ADMIN_API_KEY` は、 5. で取得したものを値として使用するため、この時点では空白

```server/.env
POSTGRES_USER=
POSTGRES_PASSWORD=
POSTGRES_DB=
POSTGRES_PORT=
MASTER_KEY=
MEILI_ENV=
MEILI_PORT=

MEILI_URL=
ADMIN_API_KEY=

DATABASE_URL=
API_URL=

BUCKET_NAME=
CLOUDFLARE_URI_ENDPOINT=
API_TOKENS_ACCESS_KEY_ID=
API_TOKENS_SECRET_ACCESS_KEY=
R2_URL=

```

4. docker の起動

```sh
#serverディレクトリ
docker-compose up -d
```

5. Default Admin API Key を取得

`<MASTER_KEY>`を設定した `MASTER_KEY` に変える

`<MEILI_URL>`を設定した `MEILI_URL` に変える

```sh
curl   -X GET '<MEILI_URL>'   -H 'Authorization: Bearer <MASTER_KEY>' | jq
```

コマンドの例

```sh
curl   -X GET 'http://localhost:7700/keys'   -H 'Authorization: Bearer master-key' | jq
```

上記のコマンドを実行すると、Default Admin API Key を含む結果が返ってくる

`server/.env`の`ADMIN_API_KEY`の値として、Default Admin API Key の値を書く

6. migrate する

```sh
DATABASE_URL="<DATABASE_URL>" sea-orm-cli migrate refresh
```

コマンドの例

```sh
DATABASE_URL="postgres://username:password@localhost:5432/db_name" sea-orm-cli migrate refresh
```

## エンティティを生成する

1. Docker の起動

```sh
#serverディレクトリ
docker-compose up -d
```

1. マイグレーションをする

```sh
DATABASE_URL="<DATABASE_URL>" sea-orm-cli migrate refresh
```

コマンドの例

```sh
DATABASE_URL="postgres://username:password@localhost:5432/db_name" sea-orm-cli migrate refresh
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

## 初期データを入れる

1. `server/src/bin/data/`に csv データを入れる

2. `server/src/bin/data/in/`に jpg ファイルを入れる

3. 以下のコマンドで実行

`sample.csv` というファイル名の場合は、以下のようになる

```sh
cargo run --bin init ./src/bin/data/sample.csv
```

## DashiShoyu の ER 図

```mermaid
erDiagram
    Item ||--|| ParentLabelJunction : "parent_visible_id"
    Item ||--|| GrandParentLabelJunction : "grand_parent_visible_id"
    ParentLabelJunction }o--|| Label : "label_id"
    GrandParentLabelJunction }o--|| Label : "label_id"
    Item |o--|| Label : "visible_id color"
    Item {
        i32 Id PK "autoincrement"
        i32 LabelId FK, UK "実際の物品ID Label Tableとリレーションを貼っている"
        i32 ParentId "DB上の親物品ID"
        String ParentLabelJunctionId FK, UK "実際の親物品ID ParentLabelJunction Tableとリレーションを貼っている"
        i32 GrandParentId "DB上の親の親物品ID"
        String GrandParentLabelJunctionId FK, UK "実際の親の親物品ID GrandParentLabelJunction Tableとリレーションを貼っている"
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
    ParentLabelJunction {
        i32 Id PK "autoincrement"
        i32 LabelId FK "Label Tableとリレーションを貼っている"
    }
    GrandParentLabelJunction {
        i32 Id PK "autoincrement"
        i32 LabelId FK "Label Tableとリレーションを貼っている"
    }
    Label {
        i32 Id PK "autoincrement"
        String VisibleId UK "実際の物品ID"
        Color Color "enum {Red, Orange, Brown, SkyBlue、Blue, Green, Yellow, Purple, Pink} (ActiveEnum)"
    }

    Object {
        i32 Id PK "autoincrement"
        String Name
        String PhotoUrl UK "Cloudflare R2に画像を格納する ファイル名は obj-{Id}.{各拡張子 MimeTypesから推測}"
        String MimeTypes
        String License
        Json Tag "e.g. ['わいわい', '楽しい場面'] (可変の配列)"
        String Description "補足説明 (空の文字列を許容する)"
        datetime CreatedAt "登録したときの日時"
        datetime UpdatedAt "更新したときの日時"
    }
```

## 最終目標の ER 図

実際の物品 ID と物品そのものの ID を分離することで、QR が剥がれても問題ないようにしている

```mermaid
erDiagram
    Item ||--o{ Transaction : transaction
    Transaction }o--|{ User : user
    Item ||--|| ParentLabelJunction : "parent_visible_id"
    Item ||--|| GrandParentLabelJunction : "grand_parent_visible_id"
    ParentLabelJunction }o--|| Label : "label_id"
    GrandParentLabelJunction }o--|| Label : "label_id"
    Item |o--|| Label : "visible_id color"
    Item {
        i32 Id PK "autoincrement"
        i32 LabelId FK, UK "実際の物品ID Label Tableとリレーションを貼っている"
        i32 ParentId "DB上の親物品ID"
        String ParentLabelJunctionId FK, UK "実際の親物品ID ParentLabelJunction Tableとリレーションを貼っている"
        i32 GrandParentId "DB上の親の親物品ID"
        String GrandParentLabelJunctionId FK, UK "実際の親の親物品ID GrandParentLabelJunction Tableとリレーションを貼っている"
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
    ParentLabelJunction {
        i32 Id PK "autoincrement"
        i32 ItemId FK, UK "Item Tableとリレーションを貼っている"
        i32 LabelId FK "Label Tableとリレーションを貼っている"
    }
    GrandParentLabelJunction {
        i32 Id PK "autoincrement"
        i32 ItemId FK, UK "Item Tableとリレーションを貼っている"
        i32 LabelId FK "Label Tableとリレーションを貼っている"
    }
    Label {
        i32 Id PK "autoincrement"
        String VisibleId UK "実際の物品ID"
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

    Object {
        i32 Id PK "autoincrement"
        String Name
        String PhotoUrl UK "Cloudflare R2に画像を格納する ファイル名は obj-{Id}.{各拡張子 MimeTypesから推測}"
        String MimeTypes
        String License
        Json Tag "e.g. ['わいわい', '楽しい場面'] (可変の配列)"
        String Description "補足説明 (空の文字列を許容する)"
        datetime CreatedAt "登録したときの日時"
        datetime UpdatedAt "更新したときの日時"
    }
```
