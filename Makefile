.PHONY: test

test:
	./scripts/test_with_mongo.sh $(ARGS)

dev:
	bacon run-long
