mdbook_build:
	cd docs \
	&& mdbook build --open

mdbook_serve:
	cd docs \
	&& mdbook serve --open

eqd_tests:
	cd edgedb-query-derive \
	&& cargo test --test lib_tests

eq_tests:
	cd edgedb-query \
	&& cargo test --test lib_tests