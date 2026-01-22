# Registre des décisions

## Navigation

- [`readme.md`](readme.md)
- [`ARCHITECTURE.md`](ARCHITECTURE.md)
- [`DECISIONS.md`](DECISIONS.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`SECURITY.md`](SECURITY.md)
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

Registre des décisions architecturales et conceptuelles

## Rôle de ce document

Ce document consigne les **décisions structurantes** prises pour le projet.

Il a pour objectifs de :

- expliciter les choix techniques, conceptuels et organisationnels majeurs ;
- éviter la rediscussion perpétuelle de décisions déjà tranchées ;
- fournir un référentiel clair aux mainteneurs et contributeurs ;
- garantir la cohérence du projet sur le long terme.

Ce document n’est **ni une roadmap**, ni une liste de fonctionnalités prévues.  
Il décrit **ce qui est décidé**, **pourquoi**, et **ce que cela implique**.

Toute contribution est évaluée à l’aune des décisions consignées ici.

---

## Gouvernance des décisions

- Les décisions sont prises dans l’intérêt **à long terme** du projet.
- Une décision peut évoluer, mais **jamais implicitement**.
- Toute remise en cause doit passer par une discussion structurée.
- La cohérence systémique prime sur l’innovation opportuniste.
- Aucune Pull Request ne peut modifier à elle seule une décision fondatrice.

---

## D-001 — Le projet est une plateforme, pas un jeu

**Statut** : Acceptée  

### Décision

Le projet est une **plateforme open-source de simulation de mondes sandbox persistants**, et **non** :

- un jeu vidéo,
- un moteur graphique,
- un RPG narratif,
- un framework de gameplay,
- une vitrine technologique.

### Justification

La valeur du projet repose sur :

- la stabilité du noyau de simulation ;
- la persistance réelle du monde ;
- la cohérence systémique ;
- la maintenabilité sur plusieurs années.

Un positionnement orienté “jeu” impose des compromis incompatibles avec ces objectifs.

### Conséquences

- Le noyau ne fournit aucun gameplay clé en main.
- L’expérience joueur n’est pas un objectif du cœur.
- Les clients sont des implémentations, jamais des piliers architecturaux.

---

## D-002 — Simulation avant narration

**Statut** : Acceptée  

### Décision

La **simulation systémique** est prioritaire sur toute forme de narration.

### Justification

Les mondes persistants crédibles produisent leurs propres récits via :

- le temps ;
- les ressources ;
- les entités ;
- les conflits ;
- les interactions.

La narration imposée affaiblit la cohérence et la crédibilité du système.

### Conséquences

- Aucun scénario, quête ou progression narrative dans le noyau.
- Toute histoire est émergente.
- Les systèmes précèdent toujours le récit.

---

## D-003 — Monde autonome et non centré sur le joueur

**Statut** : Acceptée  

### Décision

Le monde doit pouvoir **exister, évoluer et persister sans aucun joueur**.

### Justification

Un monde crédible n’a pas besoin de la présence humaine pour fonctionner.

### Conséquences

- Le serveur fonctionne sans client connecté.
- Le joueur n’a aucun statut particulier.
- Joueurs et PNJ sont soumis aux mêmes règles systémiques.

---

## D-004 — Serveur autoritaire et persistance réelle

**Statut** : Acceptée  

### Décision

Le serveur est l’unique autorité sur l’état du monde.

### Justification

La cohérence, la sécurité et la persistance exigent une source de vérité unique.

### Conséquences

- Aucune logique critique côté client.
- Persistance explicite sur disque.
- États traçables, inspectables et rejouables.
- Mode solo = serveur local.
- Mode multijoueur = serveur distant identique.

---

## D-005 — Découplage strict entre simulation et rendu

**Statut** : Acceptée  

### Décision

Le noyau de simulation est **totalement indépendant** de toute technologie de rendu ou de client.

### Justification

Le rendu est une implémentation interchangeable.  
La simulation constitue le socle durable du projet.

Lier le cœur à un moteur graphique compromettrait la portabilité et la longévité.

### Conséquences

- Aucun moteur graphique côté serveur.
- Aucun code de rendu dans le noyau.
- Le monde simulé peut être consommé par :
  - un client 3D temps réel,
  - un client 2D,
  - un client web,
  - un client headless (CLI, outils, bots, visualisation),
  - ou tout autre consommateur conforme aux APIs.
- Le client n’est jamais propriétaire de la logique du monde.

---

## D-006 — Client de rendu de référence et standard graphique officiel

**Statut** : Acceptée  

### Décision

Le projet fournit un **client de rendu de référence**, basé sur **Unreal Engine**, servant de **standard graphique officiel**, **sans exclusivité**.

### Justification

Un client de référence est nécessaire pour :

- démontrer la viabilité visuelle de la plateforme ;
- définir un standard commun d’assets et de pipeline ;
- garantir une cohérence visuelle minimale.

Cependant, aucun moteur ou client ne doit devenir une dépendance structurelle.

### Conséquences

- Unreal Engine est une **implémentation de référence**, pas une contrainte.
- D’autres clients peuvent exister librement :
  - Godot,
  - clients web,
  - clients spécialisés (administration, analyse, visualisation),
  - moteurs ou technologies futures.
- Tous les clients sont des **consommateurs du monde simulé**, jamais des décideurs.
- Le standard graphique :
  - n’impose aucune règle à la simulation ;
  - n’introduit aucune dépendance côté serveur ;
  - peut évoluer indépendamment du noyau.

> Unreal Engine n’est pas le projet.  
> C’est un client officiel parmi d’autres, remplaçable.

---

## D-007 — IA pragmatique, déterministe et explicable

**Statut** : Acceptée  

### Décision

Les entités sont des **agents déterministes**, explicables et observables.

### Justification

Un monde persistant doit être :

- débogable ;
- reproductible ;
- compréhensible.

Les IA opaques ou magiques sont incompatibles avec ces exigences.

### Conséquences

- Pas d’IA consciente ou autonome fantasmée.
- IA générative autorisée uniquement en périphérie.
- Les décisions doivent être traçables et justifiables.

---

## D-008 — Noyau minimal, extensions modulaires

**Statut** : Acceptée  

### Décision

Le noyau reste **minimal, strict et stable**.  
Toute fonctionnalité non essentielle est implémentée sous forme de **module optionnel**.

### Justification

Un noyau trop riche devient instable, rigide et coûteux à maintenir.

### Conséquences

- APIs publiques documentées et versionnées.
- Modules activables, désactivables ou remplaçables.
- Aucun module ne contourne le noyau.

---

## D-009 — Stabilité et maintenabilité avant vitesse

**Statut** : Acceptée  

### Décision

La stabilité, la lisibilité et la maintenabilité priment sur la rapidité de développement.

### Justification

Le projet vise des **années de vie**, pas une démo rapide.

### Conséquences

- Refactorisations acceptées.
- Features précipitées refusées.
- Documentation considérée comme prioritaire.

---

## Évolution des décisions

- Toute décision majeure doit être ajoutée à ce document.
- Une décision existante ne peut être modifiée qu’avec :
  - une justification explicite ;
  - une analyse d’impact ;
  - une validation des mainteneurs.
- Les décisions fondatrices ne peuvent être annulées que collectivement.

---

Fin du document.
