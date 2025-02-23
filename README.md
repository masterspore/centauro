# Centauro

# Documentació a llegir
- https://doc.rust-lang.org/book/title-page.html
- Gestió de memòria: https://deterministic.space/secret-life-of-cows.html
- Mòduls: https://stevedonovan.github.io/rust-gentle-intro/4-modules.html
- El codi de concurrència (lib.rs) està tret d'aquí: https://doc.rust-lang.org/book/ch20-02-multithreaded.html
- Estàndard dels arxius de log (ELF): https://www.w3.org/TR/WD-logfile.html

# To-Do

Prioritat alta:
- [x] ~~Funcionalitat bàsica -> Acabar HttpRequest~~
- [x] ~~Implementació multifil del servidor~~
- [x] ~~Arreglar http.rs, que el parser tiri~~
- [x] ~~Implementar loggers~~
- [ ] Implementar TLS i HTTPS
- [ ] Aconseguir CA
- [ ] Afegir capacitat del servidor per a processar POST a scripts
- [ ] Estudiar com implementar cookies/tokens: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie

Prioritat mitja:
- [x] ~~Afegir funcionalitat per a fer servir un arxiu de configuració~~
- [ ] Entendre el codi de threading
- [ ] Unit tests & integration tests
- [x] ~~Organitzar carpetes del servidor~~
- [ ] Documentació i definir estàndards de programació.
- [ ] Crear script d'instalació

Prioritat baixa:
- [x] ~~Comprar domini~~
- [ ] Implementar 2FA pel dashboard
- [ ] Implementar DNS dinàmic (Cloudflare?)

# Instalació amb SSH (Linux)
1. Descarregar servidor: 
'''
git clone https://github.com/masterspore/centauro
'''
2. Instalar Rust: https://www.rust-lang.org/tools/install, i screen:
'''
curl https://sh.rustup.rs -sSf | sh
sudo apt-get install screen
'''
3. Reinicia terminal
4. Modifica config.ini amb els valors que vulguis (de moment cal adaptar la IP)
5. Compila:
'''
cargo build
'''
6. Copia config.ini i /public:
'''
sudo cp config.ini ~/centauro/target/debug/
sudo cp -r public ~/centauro/target/debug/
'''
7. Executar servidor:
'''
screen
cd target/debug
sudo ./simple_server &
'''
8. 'Ctrl+a' i després 'd' per deixar el servidor en segon pla i poguer sortir de la sessió de ssh
9. Per agafar una altra vegada el control, 'Ctrl+a' 'r'

# Notes:
- La compilació requereix internet si no tens les dependències instal·lades (es poden veure a Cargo.toml)

# Recomnanacions pel codi:
És important comentar cada funció què fa. També s'han d'implementar unit tests i tests d'integració.

# Estàndards de programació
Peels colors de l'estil de mumuki.dev estic utilitzant la paleta següent: https://colorhunt.co/palette/153796
