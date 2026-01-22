# Roadmap – Plateforme de mondes sandbox persistants

> Cette roadmap décrit les **grandes étapes de construction** de la plateforme.  
> Elle n’est **ni exhaustive, ni contractuelle**, et peut évoluer selon les décisions de gouvernance.

## Navigation

- [`readme.md`](readme.md)
- [`ARCHITECTURE.md`](ARCHITECTURE.md)
- [`DECISIONS.md`](DECISIONS.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`SECURITY.md`](SECURITY.md)
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

La priorité absolue du projet est la **solidité du noyau de simulation**, sa **maintenabilité** et sa **pérennité** sur le long terme.

---

## Phase 0 – Fondations conceptuelles (actuelle)

🎯 **Objectif : verrouiller la vision et les invariants**

Cette phase vise à garantir que le projet repose sur des bases claires, compréhensibles et défendables dans le temps.

Livrables attendus :
- Vision et principes fondamentaux clairement documentés
- Définition explicite de ce que le projet **est** et **n’est pas**
- Séparation nette entre :
  - noyau de simulation
  - modules optionnels
  - clients
- Documentation fondatrice :
  - README
  - CONTRIBUTING
  - Code of Conduct
  - documents d’architecture initiaux

Aucune implémentation “fonctionnelle” n’est prioritaire tant que les fondations ne sont pas stabilisées.

---

## Phase 1 – Noyau de simulation minimal (serveur seul)

🎯 **Objectif : un monde qui existe sans client**

Cette phase valide le cœur du projet : un serveur capable de simuler un monde persistant **sans aucun rendu graphique**.

Fonctionnalités clés :
- Boucle de simulation déterministe
- Système de temps persistant
- Représentation de l’espace (zones / régions / chunks)
- Entités persistantes (état, identité, cycle de vie)
- Règles systémiques de base
- Persistance explicite sur disque
- Reprise après arrêt / crash
- Exécution serveur headless

À ce stade :
- aucun client graphique
- aucune UI
- aucune logique orientée joueur

Le monde doit être observable via logs, outils CLI ou dumps d’état.

---

## Phase 2 – Architecture modulaire et APIs publiques

🎯 **Objectif : permettre l’extension sans fragiliser le noyau**

Une fois le noyau minimal stable, le focus passe sur l’extensibilité contrôlée.

Axes principaux :
- Définition d’APIs publiques versionnées
- Système de modules optionnels
- Chargement / activation / désactivation de modules
- Isolation stricte entre noyau et extensions
- Gestion de compatibilité et versioning
- Documentation des invariants du noyau

Cette phase est critique pour :
- éviter le *feature creep*
- garantir la longévité du projet
- permettre une contribution communautaire saine

---

## Phase 3 – Simulation avancée et scalabilité

🎯 **Objectif : un monde crédible à grande échelle**

Le monde doit pouvoir :
- s’agrandir
- se complexifier
- survivre dans le temps

Axes de travail :
- Simulation par régions avec niveaux de détail
- Optimisation CPU / mémoire
- Gestion de grandes quantités d’entités
- Événements systémiques (économie, pénuries, migrations, conflits)
- Outils de replay et de validation déterministe
- Observabilité avancée (metrics, diagnostics)

Le focus reste **systémique**, jamais ludique.

---

## Phase 4 – Client de référence (Unreal Engine)

🎯 **Objectif : visualiser le monde, pas le définir**

Un client officiel basé sur Unreal Engine est introduit comme :
- implémentation de référence
- vitrine technique
- outil de validation visuelle

Caractéristiques :
- Consommateur strict de l’état serveur
- Aucun calcul critique côté client
- Rendu moderne mais sobre
- Standard graphique documenté
- Pipeline d’assets cohérent et extensible

Le client **ne pilote jamais** l’évolution du monde.

---

## Phase 5 – Outils, SDK et ouverture communautaire

🎯 **Objectif : faire du projet une vraie plateforme**

Dernière phase structurante avant maturité :

- SDK pour développeurs de modules
- Outils d’administration de mondes persistants
- Documentation avancée (guides, schémas, exemples)
- Templates de serveurs
- Gouvernance communautaire élargie
- Process de contribution stabilisé

Le projet devient alors une **infrastructure réutilisable**, indépendante de tout contenu officiel.

---

## Hors périmètre assumé

Cette roadmap **n’inclut pas** :
- gameplay “fun-first”
- équilibrage joueur
- narration écrite
- quêtes
- cinématiques
- contenu par défaut
- monétisation
- promesses marketing

Ces éléments relèvent des **mondes créés à partir de la plateforme**, pas de la plateforme elle-même.

---

## Principe directeur

Chaque étape est validée par une question unique :

> **Le monde peut-il exister, évoluer et persister sans joueur ni client ?**

Si la réponse est non, la fondation n’est pas encore suffisante.
