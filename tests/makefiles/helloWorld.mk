#none:

.PHONY : never hello ferris

hello:
	echo "Hello, World!"

ferris: hello
	@echo "Hello, Ferris!"

never:
	@echo "Never!"