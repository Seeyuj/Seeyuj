# Politique de sécurité

## Objectif

Ce document définit la politique de sécurité du projet **Plateforme de mondes sandbox persistants**.

## Navigation

- [`readme.md`](readme.md)
- [`ARCHITECTURE.md`](ARCHITECTURE.md)
- [`DECISIONS.md`](DECISIONS.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`SECURITY.md`](SECURITY.md)
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

L’objectif de cette politique est de :

- protéger l’intégrité du **noyau de simulation** ;
- garantir la **cohérence et la persistance** des mondes simulés ;
- encadrer la **divulgation responsable** des vulnérabilités ;
- clarifier le **périmètre de responsabilité** du projet.

La sécurité est considérée comme une **exigence structurelle**, au même titre que la persistance, le déterminisme et la maintenabilité.

---

## Périmètre de sécurité

### Inclus dans le périmètre

La politique de sécurité couvre :

- le **noyau de simulation serveur** ;
- les mécanismes de **persistance sur disque** ;
- les APIs publiques exposées par le serveur ;
- la gestion des états du monde, entités et règles ;
- les outils officiels fournis avec le noyau (CLI, scripts, utilitaires serveur).

### Hors périmètre

Ne sont **pas** couverts par cette politique :

- les clients graphiques (Unreal, Godot, Web, etc.) ;
- les modules ou extensions tiers ;
- les règles spécifiques définies par un serveur communautaire ;
- les déploiements personnalisés (cloud, bare metal, containers) ;
- les contenus, scripts ou assets produits par des tiers.

Chaque opérateur de monde est responsable de la sécurité de son environnement d’exécution.

---

## Modèle de menace (principes)

Le projet repose sur les hypothèses suivantes :

- le **serveur est autoritaire** et ne fait confiance à aucun client ;
- le client est considéré comme **potentiellement hostile** ;
- toute donnée entrante est **non fiable par défaut** ;
- la persistance est un **actif critique** (perte ou corruption = incident majeur).

Les contributions doivent respecter ces principes et ne jamais introduire de logique affaiblissant ce modèle.

---

## Bonnes pratiques de sécurité attendues

Les contributeurs sont tenus de respecter les règles suivantes :
Ces exigences complètent celles décrites dans `CONTRIBUTING.md`.

### Côté serveur

- validation stricte des entrées ;
- absence de logique critique côté client ;
- séparation claire entre données persistantes et données transitoires ;
- refus de toute dépendance réseau non maîtrisée ;
- pas de secrets codés en dur (tokens, clés, identifiants).

### Code et architecture

- lisibilité et auditabilité du code ;
- déterminisme des systèmes critiques ;
- journalisation claire des erreurs impactant l’état du monde ;
- gestion explicite des échecs (I/O, corruption, états invalides).

Un code complexe mais opaque sera refusé, même s’il est fonctionnel.

---

## Signalement des vulnérabilités

### ⚠️ Ne pas ouvrir d’issue publique

Les vulnérabilités de sécurité **ne doivent pas** être signalées via des issues publiques, discussions ou pull requests.

Cela inclut notamment :

- corruption ou perte de données persistantes ;
- escalade de privilèges ;
- contournement de l’autorité serveur ;
- exécution de code arbitraire ;
- déni de service impactant la simulation persistante.

### Canal de signalement

Merci de signaler toute vulnérabilité de manière responsable via :

**Email :** `security@<nom-du-projet>.org`  
*(adresse à adapter avant publication)*

Pour les règles générales de contribution (issues, PR, workflow), voir `CONTRIBUTING.md` — à l’exception des vulnérabilités, qui doivent suivre ce document.

Le message doit inclure :

- une description claire du problème ;
- le périmètre impacté ;
- les étapes de reproduction si possible ;
- l’impact potentiel sur la simulation ou la persistance.

---

## Processus de traitement

Les mainteneurs s’engagent à :

1. accuser réception du signalement ;
2. analyser l’impact et la sévérité ;
3. proposer un correctif ou une mitigation ;
4. publier une correction dans un délai raisonnable.

Aucune date de correction n’est garantie, mais les vulnérabilités critiques sont traitées en priorité.

---

## Divulgation responsable

Les contributeurs et chercheurs en sécurité s’engagent à :

- ne pas exploiter la vulnérabilité publiquement ;
- ne pas divulguer de détails avant correction ;
- coopérer avec les mainteneurs si nécessaire.

Toute divulgation prématurée ou exploit public volontaire peut entraîner une exclusion du projet.

---

## Versions supportées

Le projet étant en phase de fondation :

- seule la **branche principale active** est supportée ;
- aucune rétrocompatibilité de sécurité n’est garantie sur les versions obsolètes ;
- les correctifs sont appliqués sur la version en cours de développement.

Les opérateurs de mondes sont responsables de maintenir leur instance à jour.

---

## Responsabilité des opérateurs de serveurs

Les administrateurs de serveurs doivent :

- sécuriser leur système d’exploitation ;
- contrôler les accès réseau ;
- gérer les sauvegardes hors ligne ;
- auditer les modules tiers utilisés ;
- surveiller les logs et métriques.

Le projet ne saurait être tenu responsable d’un incident lié à une mauvaise configuration ou à un module externe.

---

## Philosophie

La sécurité du projet repose sur un principe simple :

> **Un monde persistant corrompu est pire qu’un monde indisponible.**

Toute décision de sécurité privilégiera :
- l’intégrité des données ;
- la cohérence de la simulation ;
- la robustesse à long terme.

La sécurité n’est pas une fonctionnalité optionnelle.
