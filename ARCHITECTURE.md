# Architecture

## Navigation

- [`readme.md`](readme.md)
- [`ARCHITECTURE.md`](ARCHITECTURE.md)
- [`DECISIONS.md`](DECISIONS.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`SECURITY.md`](SECURITY.md)
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

## Objectif du document

Ce document décrit l’architecture fondamentale de la plateforme de mondes sandbox persistants.

Il définit :
- les responsabilités de chaque couche du système ;
- les frontières architecturales non négociables ;
- les contraintes techniques imposées aux contributions ;
- les choix structurants garantissant cohérence, persistance et maintenabilité à long terme.

Ce document fait autorité.  
Toute proposition ou contribution incompatible avec cette architecture est refusée.

---

## Vision d’ensemble

La plateforme est conçue comme une **infrastructure de simulation**, et non comme un jeu.

Elle repose sur une séparation stricte entre :

1. le noyau de simulation serveur (core)  
2. les modules et extensions optionnels  
3. les clients (rendu, UI, interaction)  
4. les outils d’exploitation et d’observabilité  

Le serveur est toujours l’unique autorité.  
Le monde existe et évolue indépendamment de toute présence de client ou de joueur.

---

## Principe fondamental : serveur autoritaire

### Règle absolue

Le serveur est l’unique source de vérité de l’état du monde.

Cela implique :

- aucune décision de simulation côté client ;
- aucune logique critique exécutée hors serveur ;
- aucune persistance implicite ou recalculée ;
- aucune dépendance serveur vers un moteur graphique.

Mode solo = serveur local  
Mode multijoueur = serveur distant  

Même architecture.  
Mêmes règles.  
Aucune exception.

---

## Architecture en couches

### 1. Noyau de simulation (Core)

Le noyau est volontairement :

- minimal ;
- déterministe ;
- stable ;
- découplé ;
- maintenable sur plusieurs années.

Il ne contient **que ce qui est strictement nécessaire** à la simulation persistante d’un monde autonome.

#### Responsabilités du noyau

- gestion du temps simulé ;
- représentation de l’espace (régions, chunks, topologie) ;
- entités persistantes (agents, objets, structures) ;
- systèmes de règles (économie, besoins, production, conflits, flux) ;
- événements systémiques ;
- persistance explicite sur disque ;
- reprise après arrêt ou crash ;
- APIs publiques, documentées et versionnées.

#### Ce que le noyau ne fait jamais

- rendu graphique ;
- interface utilisateur ;
- gameplay orienté “fun” ;
- narration, quêtes ou scripts scénarisés ;
- logique spécifique ou privilégiée pour le joueur ;
- équilibrage ludique ;
- IA générative centrale.

Si une fonctionnalité n’est pas indispensable à la **simulation autonome et persistante du monde**, elle n’a pas sa place dans le noyau.

---

### 2. Modules et extensions

Les fonctionnalités non essentielles sont implémentées sous forme de **modules optionnels**.

#### Propriétés des modules

Un module :

- utilise uniquement les APIs publiques du noyau ;
- ne contourne jamais le core ;
- peut être activé ou désactivé sans compromettre le monde ;
- est versionné indépendamment ;
- peut être remplacé par une autre implémentation.

Exemples de modules possibles :

- systèmes économiques alternatifs ;
- règles sociales ou politiques ;
- IA comportementale avancée ;
- outils d’analyse ou de replay ;
- intégrations externes.

Un module ne doit **jamais** :

- modifier le noyau implicitement ;
- introduire une dépendance graphique côté serveur ;
- casser le déterminisme du core.

---

### 3. Clients (consommateurs du monde)

Les clients sont des **consommateurs de l’état du monde**, jamais des décideurs.

#### Rôle des clients

- afficher l’état du monde ;
- permettre l’interaction utilisateur ;
- transmettre des intentions au serveur ;
- proposer un rendu visuel ou textuel.

#### Découplage strict

Le noyau :

- ne connaît aucun moteur graphique ;
- n’importe aucune dépendance client ;
- peut fonctionner entièrement en mode headless.

Un client officiel basé sur Unreal Engine peut exister comme implémentation de référence, mais :

- il n’a aucun privilège ;
- il est interchangeable ;
- il ne dicte aucune règle de simulation.

D’autres clients peuvent coexister :
- Godot
- Web
- CLI
- outils de visualisation spécialisés

---

### 4. Outils d’exploitation

Les outils ne font pas partie du noyau, mais sont essentiels à la viabilité du projet.

Ils peuvent inclure :

- administration serveur ;
- inspection de l’état du monde ;
- visualisation des systèmes ;
- métriques et profiling ;
- validation de la persistance ;
- replay déterministe.

Ils respectent les mêmes contraintes :

- aucune autorité sur le monde ;
- aucune logique critique masquée ;
- interaction via APIs contrôlées.

---

## Simulation et déterminisme

### Déterminisme obligatoire

À entrée égale, le noyau doit produire :

- les mêmes décisions ;
- les mêmes transitions d’état ;
- les mêmes résultats.

Le déterminisme est une exigence fonctionnelle, pas une optimisation.

Toute source de non-déterminisme doit être :

- explicitement isolée ;
- documentée ;
- optionnelle ;
- jamais centrale.

---

## Persistance

### Persistance réelle, explicite et traçable

La persistance :

- est écrite explicitement sur disque ;
- survit aux redémarrages et crashs ;
- ne dépend pas d’états implicites ;
- peut être inspectée ou rejouée.

Le monde ne disparaît jamais lorsque le serveur s’arrête.

Toute logique reposant sur :

- mémoire volatile ;
- recalculs implicites ;
- états temporaires non sauvegardés ;

est interdite dans le noyau.

---

## Scalabilité et découpage spatial

Le monde est simulé par :

- régions ;
- chunks ;
- niveaux de détail de simulation.

Le noyau doit pouvoir :

- simuler partiellement le monde ;
- charger et décharger des zones ;
- adapter le coût de calcul ;
- fonctionner sans clients connectés.

La scalabilité est une **propriété structurelle**, pas une optimisation tardive.

---

## Contraintes techniques non négociables

- serveur headless obligatoire ;
- aucune dépendance graphique serveur ;
- APIs publiques stables et versionnées ;
- séparation stricte core / modules ;
- lisibilité et maintenabilité prioritaires ;
- refus des abstractions opaques ou “magiques” ;
- refus des dépendances fermées non justifiées.

---

## Philosophie long terme

Cette architecture est conçue pour :

- durer des années ;
- supporter des mondes très différents ;
- survivre aux évolutions technologiques ;
- permettre une gouvernance claire ;
- éviter l’explosion de complexité.

La plateforme prime toujours sur le contenu.  
La cohérence prime toujours sur la rapidité.  
Le monde prime toujours sur le joueur.

---

## Règle finale

Toute proposition est évaluée selon une seule question :

**Cette fonctionnalité est-elle nécessaire au fonctionnement d’un monde autonome, persistant et cohérent ?**

Si la réponse est non, elle n’appartient pas à cette architecture.
