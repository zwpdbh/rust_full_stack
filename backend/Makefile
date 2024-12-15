# Setup SeaORM entities for Seaography 
# Seaography is a GraphQL framework for building GraphQL resolvers using SeaORM entities.
generate_seaorm:
	sea-orm-cli generate entity -o src/models/_entities -u postgres://loco:loco@localhost:5432/loco_hello_development --seaography

# Below commands need `cargo install sea-orm-cli`
# -- generate some resource for API
generate_post:
	cargo loco generate scaffold post title:string content:text --api
	
# -- quick dev 
dev_posts:
	make reset_db && cargo run --example quick_dev_for_posts