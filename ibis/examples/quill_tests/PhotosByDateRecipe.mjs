/**
 * Copyright 2022 Google LLC
 *
 * Use of this source code is governed by a BSD-style
 * license that can be found in the LICENSE file or at
 * https://developers.google.com/open-source/licenses/bsd
 */

const Histogram = {
  $kind: '$local/Histogram',
  $outputs: [
      {url: 'hoverUrl'}
  ]
};

const Exif = {
  $kind: '$local/Exif',
  $outputs: [
      {url: 'hoverUrl'}
  ]
};

export const PhotosByDateRecipe = {
  $meta: {
    description: 'Local Photos'
  },
  $stores: {
    photos: {
      $type: `[Key]`,
      $tags: ['simple']
    },
    pickedPhoto: {
      $type: `PhotoKey`
    },
    urlWithStars: {
      $type: '[URL]'
    },
    hoverUrl: {
      $type: 'String'
    }
  },
  main: {
    $kind: '$local/PhotosByDate',
    $inputs: [
        {photos: 'photos'},
        {hoverUrl: 'hoverUrl'}
    ],
    $outputs: [
        {pickedPhoto: 'pickedPhoto'},
        {urlWithStars: 'urlWithStars'}
    ],
    $slots: {
      pickedPhotoRecipe: {
        pickedPhotoParticle: {
          $kind: '$local/PickedPhoto',
          $outputs: [
              {pickedPhoto: 'pickedPhoto'}
          ]
        }
      },
      histogram: {
        Histogram
      },
      exif: {
        Exif
      }
    }
  }
};
