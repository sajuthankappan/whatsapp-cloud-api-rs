# whatsapp-cloud-api-rs

Whatsapp Cloud API Rust Client 

## Usage example

```rust
let contentful_client = ContentfulClient::new("<access_token>", "<space_id>");
let product = contentful_client.get_entry::<Product>("<entry_id>").await?;

if let Some(product) = product {
    let name = product.name;
    //..
}
```
For more examples, please see the [tests] folder

## Querying for content

### Get a single entry

To get a single entry use the get_entry<T> method.

```rust
let product = contentful_client.get_entry::<Product>("<entry_id>").await?;

```

### Get multiple entries

There are several methods to retrieve multiple entries available in the SDK.

#### Get and filter entries

```rust
let builder = QueryBuilder::new()
        .content_type_is("product")
        .field_equals("fields.name", name);
let products = contentful_client
        .get_entries::<Product>(Some(builder))
        .await?
```

[tests]: https://github.com/sajuthankappan/contentful-rs/tree/master/tests