# Plateforme de mondes sandbox persistants

> Infrastructure open-source de simulation de mondes autonomes et persistants.

Ce dépôt contient le **noyau de simulation** d’une plateforme permettant de construire des mondes sandbox vivants, cohérents et durables.

## Navigation

- [`readme.md`](readme.md)
- [`ARCHITECTURE.md`](ARCHITECTURE.md)
- [`DECISIONS.md`](DECISIONS.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`SECURITY.md`](SECURITY.md)
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

Ce projet **n’est pas** :

- un jeu vidéo
- un moteur graphique
- un RPG narratif
- un framework de gameplay
- une vitrine technologique

---

## Objectif du projet

L’objectif est de fournir un **socle logiciel stable**, maintenable sur le long terme, permettant à des développeurs, créateurs ou communautés de bâtir **leurs propres mondes persistants**, chacun avec leurs règles, leurs systèmes et leurs usages.

Le projet se positionne comme une **infrastructure**, au même titre qu’un système d’exploitation ou un orchestrateur distribué.

> Le contenu, le gameplay et l’esthétique sont hors du périmètre du noyau.

---

## Principes fondamentaux

Ces principes sont **non négociables**.  
Toute contribution ou proposition qui les viole est rejetée.

### Monde autonome

- Le monde existe indépendamment des joueurs
- Le serveur peut fonctionner sans aucun client connecté
- Aucun scénario central
- Aucun héros
- Les joueurs sont des agents parmi d’autres

Le monde évolue par ses propres règles.

---

### Simulation avant narration

- Le cœur du projet est une simulation systémique
- Temps, espace, entités et règles sont modélisés explicitement
- La narration est émergente
- Aucun contenu narratif n’est codé dans le noyau

Si une histoire existe, elle est le résultat du système.

---

### Serveur autoritaire

- Le serveur est la seule source de vérité
- Le client ne décide jamais de l’état du monde
- Mode solo = serveur local
- Mode multijoueur = serveur distant
- Persistance réelle sur disque
- Simulation par régions avec niveaux de détail

Le client consomme l’état du monde, il ne le définit pas.

---

### Découplage strict simulation / rendu

- Le noyau ne connaît aucun moteur graphique
- Le client officiel utilise Unreal Engine
- D’autres clients peuvent exister (Godot, Web, CLI)
- Le rendu est interchangeable

Le visuel est une implémentation, pas une dépendance.

---

### Liberté encadrée

- Les mondes sont définis par des systèmes et des règles
- Les extensions passent par des APIs versionnées
- La compatibilité et la stabilité priment sur la liberté totale

La liberté existe **dans le cadre du système**, jamais en dehors.

---

### IA pragmatique

- Pas d’IA consciente
- Pas de promesse marketing
- Les agents sont déterministes et explicables
- L’IA générative est optionnelle et périphérique

L’IA est un outil, pas un pilier architectural.

---

## Ce que fournit le noyau

Le noyau est volontairement **minimal et strict**.

Il fournit :

- un système de temps persistant
- une représentation de l’espace simulé
- des entités persistantes
- des systèmes de règles modulaires
- des événements systémiques
- un mécanisme de sauvegarde et de reprise
- des APIs documentées et versionnées
- une simulation fonctionnelle sans client

---

## Ce que le noyau ne fera jamais

Le noyau **ne contient pas** :

- d’interface utilisateur
- de HUD
- de quêtes
- de dialogues écrits
- de contenu narratif
- d’assets graphiques
- d’équilibrage orienté “fun”
- de tutoriels joueur
- d’IA générative centrale

Si une fonctionnalité n’est pas nécessaire à la simulation du monde, elle n’a pas sa place ici.

---

## Standard graphique (client officiel)

Un client officiel basé sur **Unreal Engine** est fourni comme implémentation de référence.

Objectifs :

- rendu moderne (PBR, éclairage crédible)
- pipeline d’assets professionnel
- qualité visuelle élevée mais scalable
- compatibilité avec le contenu communautaire

Contraintes :

- pas de photoréalisme fragile
- pas de cinématiques imposées
- pas de narration visuelle lourde

Le client est une vitrine technique, pas le cœur du projet.

---

## Gouvernance

Ce projet est open-source, mais **pas sans gouvernance**.

- Mainteneurs du noyau identifiés
- Responsabilités clairement définies
- Process de revue strict
- Documentation prioritaire
- Refus assumés
- Stabilité > rapidité

Le projet privilégie la cohérence et la longévité à la popularité.

---

## Contribuer

Avant toute contribution :

1. Lire ce README intégralement
2. Accepter les principes fondamentaux
3. Comprendre que certaines idées seront refusées

Les règles détaillées sont définies dans :

- `CONTRIBUTING.md`
- `ARCHITECTURE.md`
- `DECISIONS.md`

---

## Règle de décision

Toute proposition est évaluée selon une question unique :

> **Cette fonctionnalité est-elle nécessaire à la simulation persistante du monde ?**

Si la réponse est non, elle n’appartient pas au noyau.

---

## Statut du projet

Le projet est en phase de fondation.  
La priorité actuelle est la **solidité conceptuelle et architecturale**, pas la vitesse de développement.
