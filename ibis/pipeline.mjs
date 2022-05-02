export const Pipeline = {
  $meta: {
    name: 'sampleBodySegmentation'
  },
  $stores: {
    image: {
      $type: 'Image',
      $tags: ['private'],
    },
    people: {
      $type: 'MaskImage'
    },
  },
  camera: {
    $kind: '$app/Library/InputCamera',
    $outputs: ['image']
  },
  bodySegmentation: {
    $kind: '$app/Library/BodySegmentation',
    $inputs: [{'image': 'image'}],
    $outputs: ['people']
  },
  image: {
    $kind: '$app/Library/OutputImage',
    $inputs: ['people'],
  }
};

export const all = {
    Pipeline
};
