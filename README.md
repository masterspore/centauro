# Centauro

# Documentació a llegir
- https://doc.rust-lang.org/book/title-page.html
- Gestió de memòria: https://deterministic.space/secret-life-of-cows.html
- Mòduls: https://stevedonovan.github.io/rust-gentle-intro/4-modules.html
- El codi de concurrència (lib.rs) està tret d'aquí: https://doc.rust-lang.org/book/ch20-02-multithreaded.html

# To-Do

Prioritat alta:
- [x] ~~Funcionalitat bàsica -> Acabar HttpRequest~~
- [x] ~~Implementació multifil del servidor~~
- [ ] Implementar loggers -> *Jan*
- [ ] Documentació i definir estàndards de programació.

Prioritat mitja:
- [ ] Entendre el codi de threading
- [ ] Unit tests & integration tests
- [ ] Organitzar carpetes del servidor
- [ ] Afegir funcionalitat per a fer servir un arxiu de configuració

Prioritat baixa:
- [ ] Comprar domini
- [ ] Implementar DNS dinàmic (Cloudflare?)

# Recomnanacions pel codi:
És important comentar cada funció què fa. També s'han d'implementar unit tests i tests d'integració.