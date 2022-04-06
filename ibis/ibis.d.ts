// Copyright 2022 Google LLC
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

export function loadIbis(
  ibis_path: string,
  status_callback: (status: string, kind: string) => void,
  version_info_callback: (version_info: string) => void
): Promise<void>;
export function check_is_subtype(
  subtype: string,
  supertype: string,
  subtypes: [string, string][]
): boolean;
export function best_solutions_to_json(input: string): string;
export function best_solutions_to_dot(input: string): string;
