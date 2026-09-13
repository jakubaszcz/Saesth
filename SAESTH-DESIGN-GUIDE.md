# Saesth — Guide de design pour le site

> Référence issue du code de l’application au 13 septembre 2026, pour la version 1.1.2. Ce document peut être transmis seul à un agent chargé du site. Les valeurs « application » décrivent l’existant ; les adaptations « site » sont des recommandations, pas des écrans déjà implémentés. Aucune validation visuelle du site n’est impliquée.

## 1. Direction artistique

Saesth est un espace calme pour composer une ambiance sonore. Son interface est nocturne, douce, cosy et accueillante : fond bleu ardoise, panneaux légèrement plus clairs, texte bleu très pâle, contours fins et angles généreusement arrondis.

Le site doit donner l’impression de prolonger l’application. La hiérarchie vient des espaces, de la typographie et de variations de surfaces discrètes. Garder une seule famille de bleus peu saturés pour l’interface générale. La palette jaune `warning` est réservée aux avertissements utiles, comme les permissions Wayland. Le côté cosy vient du rythme posé et des formes, pas d’une nouvelle palette décorative beige ou orange.

À préserver : sobriété, douceur, lisibilité, simplicité, formulations humaines. Éviter les néons, effets de verre prononcés, ombres profondes, fonds noir pur, gradients multicolores, effets 3D, particules et animations permanentes.

## 2. Palette exacte

| Token | Couleur | Usage principal |
|---|---|---|
| primary-50 | `#eef3f8` | Grands titres, texte le plus lumineux |
| primary-100 | `#dde7f1` | Texte principal, boutons, titres de cartes |
| primary-200 | `#bdd0e3` | Descriptions et texte secondaire |
| primary-300 | `#98b4d0` | Petites légendes, icônes, focus |
| primary-400 | `#7394bb` | Contour d’une carte sélectionnée, à 60 % |
| primary-500 | `#5577a3` | Interrupteur actif, certains contours |
| primary-600 | `#446084` | Bordures renforcées, survol |
| primary-700 | `#374d69` | Contours et surfaces de boutons |
| primary-800 | `#2d3f56` | Panneaux et cartes, souvent translucides |
| primary-900 | `#222f40` | Fond général, surfaces intérieures |
| primary-950 | `#121921` | Réserve sombre ; pas le fond principal actuel |

Les opacités font partie du design : `bg-primary-800/30` signifie une surface `#2d3f56` à 30 % sur le fond bleu nuit. Ne pas remplacer toutes ces surfaces par des aplats opaques.

Fond de l’application :

```css
background-color: #222f40;
background-image: radial-gradient(ellipse at top right, #374d6930, transparent 65%);
```

Halo du panneau de bienvenue :

```css
background-image: radial-gradient(
  ellipse at top right,
  rgba(152, 180, 208, 0.10),
  transparent 65%
);
```

Ce halo reste statique et discret. Il ne remplace pas le fond des autres sections.

### Palette des avertissements

Déclarée dans le bloc `@theme` de `src/App.css`, comme `primary`. Ces valeurs OKLCH conservent les jaunes du warning existant. Utiliser les classes sémantiques `warning-*`, sans couleurs `amber-*` ou `yellow-*` directement dans les composants.

| Token | Couleur exacte |
|---|---|
| warning-50 | `oklch(98.7% 0.022 95.277)` |
| warning-100 | `oklch(96.2% 0.059 95.617)` |
| warning-200 | `oklch(92.4% 0.12 95.746)` |
| warning-300 | `oklch(87.9% 0.169 91.605)` |
| warning-400 | `oklch(82.8% 0.189 84.429)` |
| warning-500 | `oklch(76.9% 0.188 70.08)` |
| warning-600 | `oklch(66.6% 0.179 58.318)` |
| warning-700 | `oklch(55.5% 0.163 48.998)` |
| warning-800 | `oklch(47.3% 0.137 46.201)` |
| warning-900 | `oklch(41.4% 0.112 45.904)` |
| warning-950 | `oklch(27.9% 0.077 45.635)` |

Warning Wayland sur Setup : panneau `quiet-panel border border-warning-400/30`, titre `text-warning-200`, corps `text-primary-200`, commande sur fond `bg-primary-900/60` avec texte sélectionnable. Le titre et les explications portent le sens de l’avertissement ; la couleur ne suffit pas. Cet avertissement apparaît uniquement lorsque la session Linux Wayland est détectée et rappelle la déconnexion/reconnexion nécessaire après modification des permissions.

## 3. Typographie

| Élément | Police | Taille dans l’app | Graisse / détails |
|---|---|---|---|
| Corps et contrôles | Nunito | 14 px le plus souvent | 400, 600 pour les actions |
| Titre de page | Outfit | 30 px / interligne 36 px | 500, espacement -0.025em |
| Titre de carte pack | Outfit | 20 px / 28 px | 500 |
| Marque Saesth | Outfit | 24 px / 32 px | 500, espacement légèrement resserré |
| Surtitre | Nunito | 12 px / 16 px | 600, capitales, espacement 0.2em |
| Description | Nunito | 14 px | Interligne environ 23 à 28 px selon le contexte |
| Légende / badge | Nunito | 12 px / 16 px | 400 à 600 |

Les fichiers de référence sont `src/assets/fonts/Nunito.ttf` et `src/assets/fonts/Outfit.ttf`, déclarés comme polices variables de graisse 100 à 900. Les fournir au projet du site et adapter leurs URL. Ne pas substituer Inter ou Manrope simplement parce que ces dépendances existent dans le dépôt.

Adaptation site : un titre de hero peut utiliser `clamp(2.25rem, 5vw, 4rem)` en Outfit 500 ; les paragraphes longs peuvent passer à 16 px pour le confort de lecture. Ces tailles sont une adaptation web, les titres de l’app restent à 30 px.

## 4. Espacement et mise en page

- Conteneur de page : centré, largeur maximale 1152 px (`max-w-6xl`).
- Marges intérieures de page : 16 px horizontal / 24 px vertical ; à partir de 640 px, 32 px horizontal / 40 px vertical.
- Espacement entre les grands blocs de l’app : 32 px.
- Grille des packs : une colonne, deux dès 768 px, trois dès 1280 px ; gouttière 16 px.
- Grille du didacticiel : une colonne, trois dès 1024 px ; gouttière 12 px.
- Cartes : padding 20 px ; panneaux communs 20 px puis 24 px dès 640 px.
- Grand panneau de bienvenue : padding 24 px puis 40 px dès 640 px.
- Rayons : 24 px pour panneaux/cartes ; 16 px pour boutons et sous-cartes ; 12 px pour petits contrôles ; pilule pour badges.
- Bordures : 1 px, translucides. Pas d’ombre dominante.

Adaptation site : prévoir 64 à 96 px entre sections sur ordinateur, 40 à 56 px sur mobile. Conserver les mêmes espaces internes aux composants. Le site doit défiler naturellement : ne pas copier le `h-screen` et les zones de défilement internes de la fenêtre desktop.

## 5. Recettes de composants fidèles

Les recettes suivantes utilisent Tailwind CSS 4, comme l’app. Les couleurs `primary`, `warning` et les polices doivent être déclarées dans le thème ; leurs noms ne sont pas fournis par Tailwind par défaut.

### Panneau standard

```text
rounded-3xl border border-primary-700/50 bg-primary-800/30 p-5 sm:p-6
```

### Bouton principal doux

```text
inline-flex items-center justify-center gap-2 rounded-2xl
border border-primary-500/30 bg-primary-700/50 px-5 py-3
text-sm font-semibold text-primary-100
transition-colors duration-300 hover:bg-primary-700
focus-visible:outline-2 focus-visible:outline-offset-4
focus-visible:outline-primary-300 motion-reduce:transition-none
```

Il peut servir au téléchargement sur le site. Pour une navigation ou un téléchargement, utiliser un lien `<a>` stylé comme un bouton, avec une vraie destination. Les `<button>` déclenchent les actions dans la page.

### Carte de pack / carte de fonctionnalité

Structure : icône en haut, titre, courte description, action en bas. Carte verticale avec hauteur étirée, `gap-5`, description dans un bloc flexible pour aligner les actions entre cartes.

- Normale : `border-primary-700/50 bg-primary-800/30`.
- Survol : `hover:border-primary-600 hover:bg-primary-800/60`.
- Sélectionnée : `border-primary-400/60 bg-primary-800/80`.
- Image du pack : 56 × 56 px, rayon 16 px, `object-cover`.
- Badge « Selected » : petite coche, fond `primary-700/60`, texte `primary-100`.
- Action en bas : largeur complète, rayon 16 px, fond `primary-900/40`, contour `primary-600/40`.

Un état sélectionné doit correspondre à une vraie sélection. Sur une carte marketing sans interaction, ne pas afficher un faux bouton « Select ».

### Étapes de découverte

Liste ordonnée de trois cartes, chacune avec une icône en haut à gauche et un numéro discret `01`, `02`, `03` à droite. Titre Nunito 14 px gras, description 14 px / 24 px. Surface `primary-900/40`, bordure `primary-700/40`, rayon 16 px.

### Sons, réglages et configuration

- Sons : cartes de la même famille que les packs ; icône dans un carré arrondi sombre de 44 px, nom en Outfit, bouton lecture/pause séparé et curseur de volume.
- Son en lecture : contour `primary-400/60`, fond `primary-800/70` ; ne pas dépendre uniquement de la couleur, conserver l’icône pause.
- Effets : panneau intérieur plus sombre, rayon 16 px ; dévoilement discret.
- Valeur de volume : petit champ arrondi avec `%`, surface `primary-900/50`, bordure `primary-700/60`.
- Interrupteurs : piste de 44 × 24 px, pouce clair de 16 px ; actif `primary-500`, inactif `primary-900`.
- Réglages : titre et description à gauche, interrupteur à droite, espace suffisant entre les deux.

Ces contrôles servent uniquement à une vraie démonstration interactive du site. Une capture de l’app peut les montrer sans créer de contrôles factices accessibles au clavier.

### Navigation

Dans l’app : barre supérieure de 72 px, logo typographique, badge du pack courant ; barre latérale de 72 px, élargie à 192 px dès 1024 px. Élément actif : surface `primary-800/80`, contour `primary-600/50`, texte clair.

Sur le site : adapter cette esthétique à une navigation horizontale et un menu mobile. Conserver logo, boutons arrondis, contours et contraste. Ne pas recopier les boutons de fermeture/minimisation, les API Tauri ni les zones de déplacement de fenêtre.

## 6. Icônes et visuels

L’app utilise `lucide-react`. Références : `Moon`, `Headphones`, `AudioLines`, `Package`, `FolderOpen`, `SlidersHorizontal`, `Sparkles`, `ArrowUpRight`, `Check`, ainsi que les icônes de pluie, vagues et feu.

La navigation et les icônes décoratives utilisent souvent des traits fins de 1.4 à 1.6 px ; certaines icônes de boutons gardent le trait Lucide par défaut. Tailles usuelles : 16 à 20 px, jusqu’à 28 px pour une illustration d’accueil. Ne pas remplacer ces traits par des emoji.

Sur le site, privilégier une capture récente de l’app ou une reproduction fidèle des composants. Ne pas inventer une interface incompatible avec le produit. Les captures doivent être fournies séparément : ce Markdown ne contient pas les images ni les polices.

## 7. Ton et contenu

L’interface actuelle est en anglais. Sauf demande contraire, conserver l’anglais sur le site. Le ton est chaleureux, court et posé ; les actions restent explicites.

Exemples réellement présents dans l’app :

- « A space to settle into. »
- « Little collections of sound, for moments that are yours. »
- « Let the world soften. »
- « A softer kind of focus. »
- « Make Saesth yours. »
- « Find your atmosphere. Take your time. »

Utiliser ce registre avec modération. Préférer « Download for Windows » pour l’action principale à une formulation poétique ambiguë. Ne pas inventer de témoignages, chiffres, certifications ou bénéfices médicaux. Afficher uniquement les téléchargements effectivement disponibles : la portabilité du design ou de Tauri ne prouve pas l’existence d’une version Linux/macOS publiée.

## 8. Proposition de structure pour le site

Cette structure est une adaptation proposée, pas une page existante de l’application.

1. Navigation : marque Saesth, liens utiles, action de téléchargement.
2. Hero : surtitre court, titre Outfit, une phrase, téléchargement et aperçu de l’app.
3. Fonctionnalités : trois cartes consacrées aux ambiances, aux sons de clavier/souris et aux packs.
4. Aperçu : capture lisible ou démonstration fonctionnelle dans un panneau arrondi.
5. Prise en main : télécharger, ajouter un pack, composer son ambiance ; style du didacticiel.
6. Téléchargement : plateformes réellement disponibles, version vérifiée, lien valide.
7. Footer : marque, liens utiles et phrase discrète ; pas de multiplication de slogans.

Ne pas ajouter automatiquement tarifs, compte utilisateur, tableau de bord ou fonctionnalités que le produit n’a pas.

## 9. Mouvement, mobile et accessibilité

Transitions de couleur d’environ 300 ms, sans zoom ni rebond au survol. Pas de parallaxe ou d’animation décorative continue. Respecter `prefers-reduced-motion`.

Focus clavier : contour de 2 px en `primary-300`, décalage de 4 px. Sur le site, appliquer aussi ce style aux liens. Nommer les boutons à icône ; masquer les icônes purement décoratives aux lecteurs d’écran. Utiliser une hiérarchie de titres, un seul h1 principal et des listes ordonnées pour les étapes.

Prévoir des cibles tactiles de 44 px minimum sur le site, même si certains contrôles desktop actuels font 40 px. Vérifier les contrastes sur les surfaces finales avec leurs opacités. Ne pas masquer globalement les barres de défilement comme le fait l’app. Ne pas lancer de son automatiquement.

Vérifier le site à 375, 768, 1024 et 1440 px de large : aucun débordement horizontal, boutons lisibles, grille adaptée, titres qui reviennent naturellement à la ligne, navigation mobile fonctionnelle.

## 10. Instructions prêtes à transmettre à l’agent du site

> Implémente le site de Saesth en suivant ce guide. Reprends exactement la palette, Nunito pour le corps, Outfit pour les titres, les surfaces translucides bleu ardoise, les contours fins et les rayons de 12/16/24 px. Adapte la structure au web avec une navigation horizontale et une page naturellement défilante. Préserve le calme du produit et garde les textes en anglais. Réutilise les composants existants du site quand possible. Distingue les captures des vraies interactions ; chaque action visible doit fonctionner. N’ajoute pas de fonctionnalités, de témoignages ou de téléchargements non confirmés. Vérifie les états de survol/focus, le clavier, la réduction des animations et les vues mobile/desktop. Avant de conclure, compare visuellement le résultat à une capture actuelle de l’app si elle est disponible, et indique clairement les vérifications effectuées.

## 11. Critères de fidélité

- [ ] Fond principal `#222f40`, jamais remplacé par du noir pur.
- [ ] Nunito et Outfit réellement chargées.
- [ ] Texte principal clair, descriptions `primary-200`, légendes `primary-300`.
- [ ] Avertissements utilisant `warning-*` défini dans `App.css`, sans jaune décoratif ailleurs.
- [ ] Surfaces et bordures respectant les opacités définies.
- [ ] Cartes à rayon 24 px et boutons à rayon 16 px.
- [ ] Hiérarchie aérée, largeur de contenu cohérente.
- [ ] Icônes Lucide cohérentes, pas d’emoji décoratifs.
- [ ] Survol calme, focus visible, réduction des animations respectée.
- [ ] Contenu et liens conformes aux fonctionnalités réellement disponibles.
- [ ] Vérification visuelle mobile et desktop réalisée, ou limite explicitement signalée.

## 12. Fichiers sources dans le dépôt de l’app

- `src/App.css` : palette, polices et composants CSS communs.
- `src/App.tsx` : structure globale de la fenêtre.
- `src/component/Header.tsx` : marque, badge et barre supérieure.
- `src/component/navigation/ComponentNavigation.tsx` : navigation et états actifs.
- `src/containers/packs/ContainerPacks.tsx` : composition de référence et didacticiel.
- `src/component/cards/packs/Card.tsx` : cartes et état sélectionné.
- `src/containers/sounds/ContainerSounds.tsx` : page des sons.
- `src/component/cards/sounds/sound/SoundCard.tsx` : lecture, volume et effets.
- `src/pages/DrawSettings.tsx` : préférences.
- `src/features/setup/ComponentSetup.tsx` : configuration.

En cas d’évolution du produit, comparer ces fichiers et actualiser ce guide. Les adaptations proposées pour le site ne doivent pas être présentées comme des propriétés déjà présentes dans l’app.
