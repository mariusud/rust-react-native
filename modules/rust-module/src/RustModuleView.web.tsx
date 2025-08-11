import * as React from 'react';

import { RustModuleViewProps } from './RustModule.types';

export default function RustModuleView(props: RustModuleViewProps) {
  return (
    <div>
      <iframe
        style={{ flex: 1 }}
        src={props.url}
        onLoad={() => props.onLoad({ nativeEvent: { url: props.url } })}
      />
    </div>
  );
}
