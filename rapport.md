# Rapport Individuel Le Hen Hugo

### Avec Chautard Hugo, Jamme Julen

## Projet Rust - Jeu vidéo RPG


# Présentation du projet

## 1. Description du projet

Le projet consiste en la réalisation d’un RPG en RUST sur le parcours d'un Barman qui va devoir concocter les cocktails demandées pour vaincre le Maitre barman adverse et récupérer le badge de son arène. Avant d'avoir accès au bar (l'arène) il faudra résoudre l'énigme du vigile a l'entrée.

### Objectifs :

* Créer une interface utilisateur dynamique avec Bevy (moteur de jeu en Rust).
* Mettre en place un flow de jeu cohérent : introduction → sélection d’ingrédients → validation → phase de crafting → fin du niveau.
* Ajouter de la logique de validation de recettes et de progression de combat.


## 2. Fonctionnalités implémentées

### a) Flow général
* **Écran de démarrage** : Possibilité de créer une nouvelle partie
* **Séléction des arènes** : Choisir son arène de combat
* **Écran d’introduction** : présentation du Maître et de l’Arène.
* **Écran de combat** :
  * Affichage des HP du joueur et du boss.
  * Liste des ingrédients disponibles.
  * Sélection des ingrédients par le joueur.
  * Validation automatique des ingrédients choisis avec feedback.
  * Réduction des HP du boss en cas de cocktail correct.
* **Phase de crafting** :
  * Présentation des instructions de préparation mélangées.
  * Mécanique de tri des instructions par le joueur.
  * Validation de l’ordre choisi.
* **Écran de fin** :
  * Félicitations si victoire.
  * Bouton pour revenir à la sélection des niveaux.
### b) Techniques utilisées

* Rust avec Bevy pour le moteur de jeu.
* Système d’UI flexbox via les composants `Node`.
* Actions utilisateurs gérées via une énumération `GameButtonAction`.
* États du jeu stockés dans `GameScreenState` :

  * `show_intro_screen`
  * `show_crafting_phase`
  * `boss_hp`, `player_hp`
  * `current_crafting` (ingrédients et instructions)
* Système de nettoyage de l’UI et réinitialisation du jeu entre les niveaux.
* Chargement des données via JSON pour les maitres, arènes videurs sauvegardes

---

## 3. Difficultés rencontrées

* Gestion des états multiples dans un même `spawn_arena_combat_screen`, nécessitant l’ajout de `show_intro_screen` et `show_crafting_phase`.
* Problème de double borrow mutable avec le `parent.spawn` → nécessité de séparer les spawns.
* Synchronisation du flow de jeu : attendre que le joueur valide chaque étape avant de passer à la suivante.



## 4. Améliorations possibles

* Ajouter des animations lors des transitions (intro → combat → crafting → victoire).
* Ajouter un système de score ou de combo.
* Implémenter des niveaux avec difficulté croissante.
* Ajouter un timer pour renforcer la tension.
* Améliorer le flow du combat notamment pour les instructions des recettes
* Rajouter plus d'arènes et de maîtres dans les JSON
* Ajouter un système d'échange "Traders" pour acheter les ingrédients nécessaires au cocktails
* Ajouter des images pour les bosses, les cocktails et les arènes du jeu
* Mettre en place des classes via les aptitudes
* Mettre en place un système de niveau et d'exp

## 5. Conclusion

Ce projet a permis de mettre en œuvre plusieurs concepts clés :

* Utilisation de Bevy pour créer une interface interactive.
* Gestion fine de l’état du jeu et de la logique métier.
* Manipulation de l’UI en Rust et apprentissage de la programmation orientée événement en Rust.

# Répartition des rôles 

Les rôles ont été répartit comme ceci au début puis une fois le développement commencé, chacun a travaillé sur différents thèmes.

## Chautard

Initialisation du projet + Mise en place des interfaces graphiques via Bevy

## Jamme

Mise en place des loaders des JSON pour le chargment des données

## Le Hen

Mise en place des premières entitées + mise en place du flow du jeu



### Hugo - Le Hen

Création des différentes classes des différents modèles lors du début du projet

Création des premiers loader pour les fichiers JSON 

Première version d'UI avec RATATA UI

Écran de précombat qui affiche le maitre et ses statistiques 

Flow de combat et de fin de combat