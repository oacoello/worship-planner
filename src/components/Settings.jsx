import React from 'react';
import {
  Title1,
  Title3,
  Card,
  CardHeader,
  makeStyles,
  tokens,
  RadioGroup,
  Radio,
  Divider,
  Badge
} from '@fluentui/react-components';
import { 
  WeatherMoonRegular, 
  WeatherSunnyRegular, 
  DesktopRegular, 
  LocalLanguageRegular, 
  ColorRegular 
} from '@fluentui/react-icons';
import { useAppTheme } from '../themeContext';
import { useI18n } from '../i18n';

const useStyles = makeStyles({
  container: {
    padding: '32px',
    maxWidth: '900px',
    margin: '0 auto',
    display: 'flex',
    flexDirection: 'column',
    gap: '24px',
  },
  header: {
    marginBottom: '12px',
  },
  sectionCard: {
    padding: '24px',
    backgroundColor: tokens.colorNeutralBackground1,
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRadius: tokens.borderRadiusMedium,
    boxShadow: tokens.shadow4,
    display: 'flex',
    flexDirection: 'column',
    gap: '16px'
  },
  optionRow: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    padding: '12px 16px',
    borderRadius: tokens.borderRadiusMedium,
    backgroundColor: tokens.colorNeutralBackground2,
    borderTop: `1px solid ${tokens.colorNeutralStroke2}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke2}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke2}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke2}`,
  },
  optionLabel: {
    display: 'flex',
    alignItems: 'center',
    gap: '12px',
    fontSize: '14px',
    fontWeight: '600'
  },
  radioGroupHorizontal: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(180px, 1fr))',
    gap: '12px',
    marginTop: '8px'
  },
  themeChoiceCard: {
    display: 'flex',
    alignItems: 'center',
    gap: '12px',
    padding: '16px',
    borderRadius: tokens.borderRadiusMedium,
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    backgroundColor: tokens.colorNeutralBackground2,
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    '&:hover': {
      backgroundColor: tokens.colorNeutralBackground1Hover,
      borderTopColor: tokens.colorBrandStroke1,
      borderBottomColor: tokens.colorBrandStroke1,
      borderLeftColor: tokens.colorBrandStroke1,
      borderRightColor: tokens.colorBrandStroke1,
    }
  },
  themeChoiceActive: {
    borderTop: `2px solid ${tokens.colorBrandStroke1}`,
    borderBottom: `2px solid ${tokens.colorBrandStroke1}`,
    borderLeft: `2px solid ${tokens.colorBrandStroke1}`,
    borderRight: `2px solid ${tokens.colorBrandStroke1}`,
    backgroundColor: tokens.colorBrandBackground2,
  }
});

export default function Settings() {
  const styles = useStyles();
  const { themeMode, setThemeMode } = useAppTheme();
  const { language, setLanguage, t } = useI18n();

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <Title1>{t('settingsTitle')}</Title1>
        <div style={{ color: tokens.colorNeutralForeground3, marginTop: '4px', fontSize: '14px' }}>
          {t('settingsSubtitle')}
        </div>
      </div>

      {/* Appearance Section */}
      <div className={styles.sectionCard}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <ColorRegular fontSize={24} style={{ color: tokens.colorBrandForeground1 }} />
          <div>
            <Title3>{t('appearance')}</Title3>
            <div style={{ color: tokens.colorNeutralForeground3, fontSize: '13px', marginTop: '2px' }}>
              {t('themeDescription')}
            </div>
          </div>
        </div>

        <div className={styles.radioGroupHorizontal}>
          <div 
            className={`${styles.themeChoiceCard} ${themeMode === 'dark' ? styles.themeChoiceActive : ''}`}
            onClick={() => setThemeMode('dark')}
          >
            <WeatherMoonRegular fontSize={20} />
            <div>
              <div style={{ fontWeight: '600', fontSize: '14px' }}>{t('darkTheme')}</div>
            </div>
          </div>

          <div 
            className={`${styles.themeChoiceCard} ${themeMode === 'light' ? styles.themeChoiceActive : ''}`}
            onClick={() => setThemeMode('light')}
          >
            <WeatherSunnyRegular fontSize={20} />
            <div>
              <div style={{ fontWeight: '600', fontSize: '14px' }}>{t('lightTheme')}</div>
            </div>
          </div>

          <div 
            className={`${styles.themeChoiceCard} ${themeMode === 'system' ? styles.themeChoiceActive : ''}`}
            onClick={() => setThemeMode('system')}
          >
            <DesktopRegular fontSize={20} />
            <div>
              <div style={{ fontWeight: '600', fontSize: '14px' }}>{t('systemTheme')}</div>
            </div>
          </div>
        </div>
      </div>

      {/* Language Section */}
      <div className={styles.sectionCard}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <LocalLanguageRegular fontSize={24} style={{ color: tokens.colorBrandForeground1 }} />
          <div>
            <Title3>{t('language')}</Title3>
            <div style={{ color: tokens.colorNeutralForeground3, fontSize: '13px', marginTop: '2px' }}>
              {t('languageDescription')}
            </div>
          </div>
        </div>

        <div className={styles.radioGroupHorizontal}>
          <div 
            className={`${styles.themeChoiceCard} ${language === 'es' ? styles.themeChoiceActive : ''}`}
            onClick={() => setLanguage('es')}
          >
            <div>
              <div style={{ fontWeight: '600', fontSize: '14px' }}>Español</div>
              <div style={{ fontSize: '12px', color: tokens.colorNeutralForeground3 }}>Español (Latinoamérica)</div>
            </div>
          </div>

          <div 
            className={`${styles.themeChoiceCard} ${language === 'en' ? styles.themeChoiceActive : ''}`}
            onClick={() => setLanguage('en')}
          >
            <div>
              <div style={{ fontWeight: '600', fontSize: '14px' }}>English</div>
              <div style={{ fontSize: '12px', color: tokens.colorNeutralForeground3 }}>English (United States)</div>
            </div>
          </div>
        </div>
      </div>

      {/* System & Copyright Information */}
      <div style={{ textAlign: 'center', marginTop: '40px', color: tokens.colorNeutralForeground4, fontSize: '12px' }}>
        © {new Date().getFullYear()} Granco Planner • Todos los derechos reservados, Oscar Coello
      </div>
    </div>
  );
}
