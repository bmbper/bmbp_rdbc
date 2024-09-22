echo "publish bmbp_rdbc..."
cd crates
cd bmbp_rdbc_type
cargo publish
cd ../bmbp_rdbc_sql 
cargo publish
cd ../bmbp_rdbc_orm
cargo publish
cd ../bmbp_rdbc_script
cargo publish
cd ../bmbp_rdbc_marco
cargo publish
cd ../bmbp_rdbc_driver
cargo publish
cd ../bmbp_rdbc_adaptor
cargo publish
cd ../bmbp_rdbc
cargo publish
echo "publish bmbp_rdbc success"