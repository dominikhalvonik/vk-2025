# vk-2025

# Docker CLI

Vytvorenie Image s názvom vk-apache na základe Dockerfile s názvom Dockerfile-apache pričom
základný kontext pre build je priečinok v ktorom sa nachádzam čo reprezentuje tá podka na konci

``docker build -t vk-apache -f config/Dockerfile-apache .``

Vytvorenie kontaneru z image vk-apache, namapovanie portov tak, že vnútorný port kontajenera 80
sa presmeruje na vonkajsi port hostitela 8080, -d znamena, ze vystup s konzoli kontanera sa nebude zobrazovat
v nasej konzole, cize nastane tzv. "detach"

``docker run -d -p 8080:80 vk-apache``

Zobrazenie vsetkych beziacich kontajnerov

``docker ps``

Zastavenie kontajnera s nazvom trusting_shamir

``docker stop trusting_shamir``

Vymazanie kontajnera s názvom trusting_shamir

``docker rm trusting_shamir``

Zobrzenie všetkých lokálnych images na disku

``docker images``
