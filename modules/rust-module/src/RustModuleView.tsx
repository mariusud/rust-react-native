import { requireNativeView } from 'expo';
import * as React from 'react';

import { RustModuleViewProps } from './RustModule.types';

const NativeView: React.ComponentType<RustModuleViewProps> =
  requireNativeView('RustModule');

export default function RustModuleView(props: RustModuleViewProps) {
  return <NativeView {...props} />;
}
