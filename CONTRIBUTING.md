# Guide de contribution

Merci de l’intérêt que vous portez à ce projet.

Avant toute contribution (code, documentation, idées, issues, discussions), merci de lire le `readme.md` et de vous assurer que votre proposition est alignée avec l’objectif du dépôt : **fournir un noyau de simulation de mondes sandbox persistants**, maintenable sur le long terme.

Ce dépôt n’est ni un jeu vidéo, ni un moteur graphique, ni un RPG narratif. La priorité du projet est la **cohérence systémique**, la **stabilité du noyau**, la **persistance réelle** du monde simulé et la **maintenabilité sur plusieurs années**.

Certaines contributions, même techniquement correctes, peuvent être refusées si elles ne servent pas la vision et l’architecture du projet.

## Navigation

- [`readme.md`](readme.md)
- [`ARCHITECTURE.md`](ARCHITECTURE.md)
- [`DECISIONS.md`](DECISIONS.md)
- [`ROADMAP.md`](ROADMAP.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`SECURITY.md`](SECURITY.md)
- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

## Table des matières

- [Avant de contribuer](#avant-de-contribuer)
- [Sécurité (signalement responsable)](#securite-signalement-responsable)
- [Philosophie et principes du projet](#philosophie-et-principes-du-projet)
  - [Simulation avant narration](#simulation-avant-narration)
  - [Monde autonome et non centre sur le joueur](#monde-autonome-et-non-centre-sur-le-joueur)
  - [Serveur autoritaire et persistance reelle](#serveur-autoritaire-et-persistance-reelle)
  - [IA pragmatique deterministe et explicable](#ia-pragmatique-deterministe-et-explicable)
  - [Modularite stricte et extensibilite controlee](#modularite-stricte-et-extensibilite-controlee)
  - [Separation stricte entre simulation et rendu](#separation-stricte-entre-simulation-et-rendu)
  - [Plateforme avant contenu](#plateforme-avant-contenu)
- [Types de contributions acceptees](#types-de-contributions-acceptees)
- [Contributions explicitement refusees](#contributions-explicitement-refusees)
- [Architecture regles techniques et contraintes](#architecture-regles-techniques-et-contraintes)
- [Workflow de contribution](#workflow-de-contribution)
- [Processus de validation et gouvernance](#processus-de-validation-et-gouvernance)
- [Code de conduite et communication](#code-de-conduite-et-communication)

## Avant de contribuer

En contribuant à ce dépôt, vous acceptez que :

- la **simulation** et la **persistance** priment sur la narration ou le rendu ;
- le **serveur** reste l’unique autorité sur l’état du monde ;
- les systèmes doivent être **explicables**, **déterministes** et **découplés** ;
- la gouvernance du projet privilégie la **cohérence** plutôt que la rapidité.

Si ces principes ne correspondent pas à vos attentes ou à votre approche, ce projet n’est probablement pas adapté à votre contribution — et ce choix est parfaitement légitime.

Pour maximiser les chances d’acceptation :

- **Pour une modification significative** (nouvelle fonctionnalité, changement du noyau, refactorisation majeure), ouvrez d’abord une issue ou une discussion. *(Exception : vulnérabilités de sécurité — voir `SECURITY.md`.)*
- **Une Pull Request = un objectif** : évitez les changements “fourre-tout”.
- **Expliquez le pourquoi** : le contexte et l’impact systémique comptent autant que le code.

## Sécurité (signalement responsable)

Les vulnérabilités de sécurité **ne doivent pas** être signalées via des issues publiques, discussions ou pull requests.

Merci de suivre la procédure et le canal de signalement décrits dans `SECURITY.md`.

## Philosophie et principes du projet

Cette plateforme repose sur un ensemble de principes structurants qui guident l’ensemble des décisions techniques, architecturales et organisationnelles. Toute contribution est évaluée à l’aune de ces principes.

### Simulation avant narration

Le cœur du projet est un moteur de **simulation persistante**.

Le monde existe, évolue et se transforme indépendamment de toute présence humaine ou joueur. Aucun scénario, aucune narration imposée, aucun “parcours” prédéfini ne structure le fonctionnement du monde.

Les récits, histoires et situations émergent exclusivement des systèmes simulés (temps, ressources, entités, conflits, interactions).

Toute contribution introduisant une logique narrative imposée, linéaire ou centrée sur le joueur sera refusée.

### Monde autonome et non centre sur le joueur

Le joueur n’est jamais une entité centrale, indispensable ou privilégiée.

Le monde :

- continue d’évoluer en l’absence de joueurs ;
- ne se met pas en pause ;
- ne s’adapte pas artificiellement à la présence humaine.

Les joueurs, comme les PNJ, sont des acteurs parmi d’autres, soumis aux mêmes règles systémiques.

Toute contribution supposant un “héros”, un rôle unique, ou un traitement spécial du joueur est incompatible avec ce projet.

### Serveur autoritaire et persistance reelle

Le serveur est l’unique autorité sur l’état du monde.

Cela implique :

- aucune logique critique ne doit être exécutée côté client ;
- le client est un consommateur de l’état du monde, jamais un décideur ;
- la persistance est assurée sur disque, de manière explicite et traçable.

Les mécanismes reposant sur des états volatiles, temporaires ou implicitement recalculés sont proscrits pour le noyau de simulation.

### IA pragmatique deterministe et explicable

Les entités non-joueurs sont des **agents** définis par :

- des besoins ;
- des objectifs ;
- une mémoire ;
- des règles de décision explicables.

Le projet ne vise pas à créer une IA “magique” ou opaque. Les comportements doivent être reproductibles, observables et débogables.

Les approches probabilistes ou génératives peuvent exister **en périphérie**, mais jamais au cœur de la simulation persistante.

### Modularite stricte et extensibilite controlee

Le noyau du projet doit rester :

- minimal ;
- stable ;
- découplé ;
- maintenable sur le long terme.

Toute fonctionnalité non essentielle doit être implémentée sous forme de **module optionnel**, clairement isolé du cœur de la simulation.

Les modules :

- utilisent des APIs publiques et versionnées ;
- ne contournent pas le noyau ;
- peuvent être activés, désactivés ou remplacés sans compromettre le monde.

### Separation stricte entre simulation et rendu

La simulation du monde est totalement indépendante de toute technologie de rendu.

Le projet peut définir un standard graphique de référence, mais ce standard :

- ne dicte jamais les règles de la simulation ;
- n’introduit aucune dépendance graphique dans le serveur ;
- reste un consommateur du monde simulé.

Toute contribution couplant directement la logique de simulation à un moteur graphique sera refusée.

### Plateforme avant contenu

Le projet fournit des **systèmes**, des **règles** et des **outils**, pas des expériences clés en main.

Le contenu (mondes, factions, scripts, assets, règles spécifiques) est laissé à la charge des serveurs, des créateurs et des communautés.

Les contributions ajoutant du contenu “par défaut” au détriment de la robustesse de la plateforme ne sont pas prioritaires.

## Types de contributions acceptees

Le projet accepte et encourage les contributions qui renforcent la stabilité, la cohérence et la pérennité de la plateforme.

### Corrections de bugs

Toute correction améliorant :

- la stabilité du serveur ;
- la cohérence de la simulation ;
- la fiabilité de la persistance ;
- la reproductibilité des comportements ;

est considérée comme prioritaire.

Les corrections doivent être accompagnées d’une description claire du problème, de son impact systémique et, lorsque possible, d’un test reproduisant le cas.

### Ameliorations de performance et de scalabilite

Les contributions portant sur :

- la consommation CPU / mémoire ;
- la gestion des régions ou chunks de simulation ;
- la montée en charge serveur ;
- l’optimisation des accès disque et de la persistance ;

sont fortement encouragées.

Toute optimisation doit privilégier la lisibilité, la maintenabilité et la stabilité plutôt qu’un gain marginal isolé.

### Documentation technique

La documentation est une contribution à part entière.

Sont notamment encouragés :

- documentation de l’architecture ;
- schémas de flux de simulation ;
- description des invariants du système ;
- guides de déploiement serveur ;
- documentation des APIs publiques et des modules.

Une contribution documentaire claire est souvent plus précieuse qu’une fonctionnalité supplémentaire.

### Tests validation et observabilite

Le projet valorise les contributions liées à :

- tests unitaires et d’intégration ;
- outils de validation de la persistance ;
- mécanismes de replay ou de simulation déterministe ;
- métriques, logs structurés et outils de diagnostic.

Toute amélioration facilitant le débogage d’un monde persistant est considérée comme stratégique.

### Modules optionnels et extensions

Les fonctionnalités non essentielles au noyau peuvent être proposées sous forme de **modules optionnels**, à condition de :

- respecter strictement les APIs publiques ;
- ne pas introduire de dépendance vers le cœur ;
- rester désactivables sans impact sur la simulation.

Les modules expérimentaux sont acceptés tant qu’ils restent clairement identifiés comme tels.

### Outils de developpement et d’exploitation

Les outils facilitant :

- le développement local ;
- le profiling ;
- le monitoring serveur ;
- la gestion de mondes persistants ;
- l’administration et la maintenance ;

sont des contributions bienvenues, même s’ils ne sont pas directement visibles côté client.

## Contributions explicitement refusees

Afin de préserver la cohérence, la maintenabilité et la viabilité à long terme de la plateforme, certaines catégories de contributions sont explicitement refusées.

### Fonctionnalites orientees jeu ou fun immediat

Les contributions visant principalement à :

- améliorer l’expérience ludique à court terme ;
- ajouter des mécaniques de gameplay isolées ;
- enrichir le contenu sans impact systémique ;

ne sont pas acceptées dans le noyau du projet.

Le projet fournit des systèmes, pas des expériences ludiques clés en main.

### Narration imposee ou logique scenarisee

Toute contribution introduisant :

- des quêtes linéaires ;
- des événements scriptés obligatoires ;
- une progression narrative imposée ;

est incompatible avec le principe de narration émergente.

Les histoires doivent découler des systèmes simulés, jamais les précéder.

### Centralite ou traitement special du joueur

Les contributions supposant :

- un rôle unique ou privilégié du joueur ;
- des règles spécifiques applicables uniquement aux joueurs ;
- une adaptation artificielle du monde à la présence humaine ;

sont refusées.

Le joueur est un acteur parmi d’autres, soumis aux mêmes règles que les entités simulées.

### Logique client autoritaire ou couplee a la simulation

Toute logique critique exécutée côté client est proscrite.

Cela inclut notamment :

- décisions de simulation prises côté client ;
- calculs impactant l’état persistant du monde ;
- dépendance directe entre le client et le cœur serveur.

Le client n’est jamais une source de vérité.

### IA opaque non deterministe ou magique

Les contributions reposant sur :

- des décisions non explicables ;
- des modèles non reproductibles ;
- une dépendance forte à des systèmes génératifs externes ;

ne sont pas acceptées dans le noyau de simulation.

Les comportements doivent pouvoir être compris, observés et reproduits.

### Couplage fort avec une technologie ou un moteur specifique

Toute contribution :

- liant le cœur de la simulation à un moteur graphique ;
- introduisant une dépendance non justifiée à une technologie fermée ;
- empêchant l’exécution serveur headless ;

sera refusée.

Le projet doit rester indépendant de tout client ou moteur particulier.

### Ajout de contenu par defaut dans le noyau

Le noyau n’a pas vocation à contenir :

- des mondes prédéfinis ;
- des factions “officielles” ;
- des règles de jeu imposées ;
- des assets graphiques obligatoires.

Le contenu appartient aux serveurs, pas à la plateforme.

## Architecture regles techniques et contraintes

Toute contribution doit respecter l’architecture globale du projet et ses contraintes techniques, conçues pour garantir la stabilité, la persistance et l’évolutivité du système sur le long terme.

### Separation stricte entre noyau et extensions

Le projet est structuré autour :

- d’un **noyau de simulation minimal et stable** ;
- de **modules et extensions optionnels**.

Le noyau :

- contient exclusivement les mécanismes indispensables à la simulation persistante ;
- expose des APIs publiques, documentées et versionnées ;
- ne dépend d’aucun module externe.

Toute fonctionnalité non strictement essentielle doit être implémentée sous forme de module.

### Stabilite des APIs et gestion des breaking changes

Les APIs publiques du noyau sont considérées comme **stables**.

Par conséquent :

- toute modification incompatible doit être explicitement justifiée ;
- aucun breaking change ne sera accepté sans discussion préalable ;
- les impacts sur les modules existants doivent être clairement documentés.

La compatibilité ascendante est une priorité.

### Determinisme et reproductibilite

Les systèmes du noyau doivent être :

- déterministes à entrée égale ;
- reproductibles dans le temps ;
- observables et débogables.

Toute logique introduisant des comportements non reproductibles doit être isolée, documentée et justifiée.

### Persistance explicite et tracable

Les données persistantes du monde doivent :

- être écrites explicitement sur disque ;
- pouvoir être rejouées ou inspectées ;
- ne pas dépendre d’états implicites ou transitoires.

Les mécanismes de persistance doivent être conçus pour survivre aux redémarrages, aux crashs et aux migrations de version.

### Execution serveur headless obligatoire

Le serveur doit pouvoir fonctionner :

- sans interface graphique ;
- sans dépendance client ;
- sans moteur de rendu.

Toute contribution introduisant une dépendance graphique directe ou indirecte côté serveur sera refusée.

### Lisibilite simplicite et maintenabilite

La lisibilité du code est une exigence fonctionnelle.

Les contributions doivent privilégier :

- des abstractions simples ;
- des dépendances minimales ;
- un code compréhensible sans contexte implicite.

Un code “intelligent” mais difficile à maintenir sera rejeté au profit d’une solution plus simple et plus robuste.

## Workflow de contribution

Toute contribution au projet suit un processus standardisé afin de garantir la qualité, la cohérence et la traçabilité des changements.

### Discussion prealable recommandee

Pour toute contribution significative (nouvelle fonctionnalité, modification du noyau, refactorisation majeure), il est fortement recommandé d’ouvrir une issue ou une discussion avant d’écrire du code.

Cela permet de :

- vérifier l’alignement avec la philosophie du projet ;
- éviter du travail inutile ;
- anticiper les impacts systémiques.

### Fork et branches

Les contributions se font via un fork du dépôt principal.

Les branches doivent suivre une convention claire :

- `fix/description-courte`
- `feature/description-courte`
- `doc/description-courte`
- `refactor/description-courte`

Chaque branche doit être limitée à un objectif précis.

### Commits

Les commits doivent :

- être atomiques ;
- avoir un message clair et descriptif ;
- expliquer le *pourquoi* du changement, pas seulement le *quoi*.

Les commits correctifs ou temporaires doivent être nettoyés avant la soumission de la Pull Request.

### Pull Requests

Toute contribution passe par une Pull Request.

La Pull Request doit inclure :

- une description du problème ou de l’objectif ;
- la justification systémique du changement ;
- les impacts sur la simulation, la persistance et la compatibilité ;
- les tests ajoutés ou adaptés, le cas échéant.

Les Pull Requests incomplètes ou insuffisamment justifiées peuvent être fermées sans revue approfondie.

### Revue et iterations

Les contributions sont revues par les mainteneurs du projet.

Des modifications ou clarifications peuvent être demandées. L’absence de réponse prolongée peut entraîner la fermeture de la Pull Request.

La revue porte autant sur l’architecture que sur le code lui-même.

## Processus de validation et gouvernance

Ce projet est open-source, mais il n’est pas dépourvu de gouvernance.

Les décisions techniques et architecturales sont prises dans l’intérêt du projet à long terme, et non en fonction du volume de contributions ou de la popularité d’une proposition.

### Mainteneurs et responsabilite

Le projet est piloté par un groupe restreint de mainteneurs responsables du noyau et de la vision globale.

Les mainteneurs :

- définissent et protègent l’architecture du projet ;
- valident ou refusent les contributions ;
- garantissent la cohérence à long terme de la plateforme.

La responsabilité du noyau ne peut être déléguée implicitement par une Pull Request.

### Criteres de validation des contributions

Les contributions sont évaluées selon les critères suivants :

- alignement avec la philosophie du projet ;
- impact sur la stabilité et la persistance ;
- clarté et maintenabilité du code ;
- compatibilité avec l’architecture existante ;
- bénéfice systémique à long terme.

Une contribution techniquement correcte peut être refusée si elle ne respecte pas ces critères.

### Refus desaccords et arbitrage

Le refus d’une contribution est une décision normale et fait partie du processus de gouvernance.

Les mainteneurs peuvent refuser une contribution :

- sans obligation de proposer une alternative ;
- sans entrer dans un débat prolongé ;
- sans justification exhaustive au-delà des principes établis.

Les désaccords doivent rester techniques, argumentés et respectueux.

### Evolution du projet

La vision et les principes du projet peuvent évoluer, mais uniquement :

- de manière collective ;
- via des discussions structurées ;
- avec un impact clairement mesuré sur l’existant.

Aucune Pull Request isolée ne peut modifier à elle seule les fondations du projet.

## Code de conduite et communication

Le projet vise à maintenir un environnement de collaboration sain, respectueux et orienté vers des échanges techniques constructifs.

### Respect et professionnalisme

Toute interaction (issues, discussions, revues de code, commentaires) doit rester :

- respectueuse ;
- factuelle ;
- centrée sur le contenu technique.

Les attaques personnelles, les jugements de valeur et les comportements agressifs ne sont pas tolérés.

### Desaccords techniques

Les désaccords sont normaux et attendus dans un projet d’infrastructure.

Ils doivent :

- être argumentés techniquement ;
- s’appuyer sur des faits, des mesures ou des principes établis ;
- éviter toute personnalisation du débat.

Le désaccord n’implique jamais une remise en cause des intentions ou des compétences.

### Communication et attentes

Les mainteneurs contribuent sur leur temps et selon leurs priorités.

Il n’existe :

- aucune obligation de réponse immédiate ;
- aucune garantie d’acceptation d’une contribution ;
- aucun engagement de roadmap publique.

Toute pression, insistance ou tentative de forcer une décision est contraire à l’esprit du projet.

### Application

Les mainteneurs se réservent le droit de :

- modérer les discussions ;
- fermer des issues ou Pull Requests ;
- limiter ou suspendre l’accès aux contributions ;

lorsque le comportement d’un participant nuit au bon fonctionnement du projet.
