import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { FluentProvider } from '@fluentui/react-components'
import { CustomThemeProvider, useAppTheme } from './themeContext.jsx'
import { I18nProvider } from './i18n.jsx'
import App from './App.jsx'
import './index.css'

function ThemedApp() {
  const { currentTheme } = useAppTheme();
  return (
    <FluentProvider theme={currentTheme} style={{ height: '100vh', display: 'flex', flexDirection: 'column' }}>
      <App />
    </FluentProvider>
  );
}

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <CustomThemeProvider>
      <I18nProvider>
        <ThemedApp />
      </I18nProvider>
    </CustomThemeProvider>
  </StrictMode>,
)
