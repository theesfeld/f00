# f00 suite — primary build is pure assembly (f00-ls today)
.PHONY: all asm test clean install
all: asm
asm:
	$(MAKE) -C asm
test:
	$(MAKE) -C asm test
clean:
	$(MAKE) -C asm clean
install:
	$(MAKE) -C asm install
