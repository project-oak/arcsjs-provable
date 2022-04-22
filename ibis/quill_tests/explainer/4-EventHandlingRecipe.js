/**
 * @license
 * Copyright 2021 Google LLC.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * Code distributed by Google as part of this project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */
const local = `/arcsjs-chromium/demo/explainer/Library`;

export const EventHandlingRecipe = {
  $meta: {
    description: 'Event Handling'
  },
  $stores: {
    fonts: {
      $type: `[Key]`,
      $tags: ['simple']
    },
    pickedFont: {
      $type: `FontKey`
    }
  },
  main: {
    $kind: `${local}/4-EventHandling`,
    $bindings: {
      fonts: 'fonts',
      pickedFont: 'pickedFont'
    }
  }
};
