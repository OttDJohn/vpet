all: frontend backend

deploy: backend
	cd deploy && ansible-playbook -u root -i inventory.ini playbook.yml
frontend:
	-rm vpet.zip
	cd frontend && zip ../vpet.zip `find .`

backend:
	cd backend && cargo build
 
.PHONY: all frontend backend
