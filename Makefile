all: frontend backend

frontend:
	-rm vpet.zip
	cd frontend && zip ../vpet.zip `find .`

backend:
	cd backend && cargo build
 
.PHONY: all frontend backend
