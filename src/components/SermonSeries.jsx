import React, { useState, useEffect, useRef } from 'react';
import {
  Title1,
  Title3,
  Button,
  Card,
  CardFooter,
  makeStyles,
  tokens,
  Dialog,
  DialogTrigger,
  DialogSurface,
  DialogTitle,
  DialogBody,
  DialogActions,
  DialogContent,
  Field,
  Input,
  Textarea
} from '@fluentui/react-components';
import { AddRegular, DeleteRegular, EditRegular, ImageRegular } from '@fluentui/react-icons';
import { getSermonSeries, saveSermonSeries, deleteSermonSeries } from '../db';
import { useI18n } from '../i18n';

const useStyles = makeStyles({
  container: {
    padding: '32px',
    maxWidth: '1200px',
    margin: '0 auto',
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: '32px',
  },
  grid: {
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
    gap: '24px',
  },
  card: {
    padding: '0px',
    backgroundColor: tokens.colorNeutralBackground1,
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRadius: tokens.borderRadiusMedium,
    overflow: 'hidden',
    display: 'flex',
    flexDirection: 'column',
    transition: 'transform 0.15s ease, box-shadow 0.15s ease',
    '&:hover': {
      transform: 'translateY(-3px)',
      boxShadow: tokens.shadow16,
    }
  },
  cardBanner: {
    width: '100%',
    height: '160px',
    backgroundColor: tokens.colorNeutralBackground3,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    overflow: 'hidden',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
  },
  cardBannerImage: {
    width: '100%',
    height: '100%',
    objectFit: 'cover',
    display: 'block'
  },
  placeholderArt: {
    width: '100%',
    height: '100%',
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: tokens.colorNeutralBackground3,
    color: tokens.colorNeutralForeground3,
    gap: '8px'
  },
  cardContent: {
    padding: '16px 16px 12px',
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    flex: 1
  },
  form: {
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
    marginTop: '16px'
  },
  dropzone: {
    borderTop: `2px dashed ${tokens.colorNeutralStroke1}`,
    borderBottom: `2px dashed ${tokens.colorNeutralStroke1}`,
    borderLeft: `2px dashed ${tokens.colorNeutralStroke1}`,
    borderRight: `2px dashed ${tokens.colorNeutralStroke1}`,
    borderRadius: tokens.borderRadiusMedium,
    padding: '24px',
    textAlign: 'center',
    cursor: 'pointer',
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    gap: '8px',
    color: tokens.colorNeutralForeground3,
    transition: 'all 0.2s ease',
    '&:hover': {
      backgroundColor: tokens.colorNeutralBackground1Hover,
    }
  },
  dropzoneActive: {
    borderTop: `2px dashed ${tokens.colorBrandStroke1}`,
    borderBottom: `2px dashed ${tokens.colorBrandStroke1}`,
    borderLeft: `2px dashed ${tokens.colorBrandStroke1}`,
    borderRight: `2px dashed ${tokens.colorBrandStroke1}`,
    backgroundColor: tokens.colorBrandBackground2,
    color: tokens.colorBrandForeground1,
  },
  previewImage: {
    width: '100%',
    height: '140px',
    objectFit: 'cover',
    borderRadius: tokens.borderRadiusMedium
  },
  deleteButton: {
    color: tokens.colorPaletteRedForeground1,
    '&:hover': {
      color: tokens.colorPaletteRedForeground2,
      backgroundColor: tokens.colorPaletteRedBackground2
    }
  }
});

export default function SermonSeries() {
  const styles = useStyles();
  const { t } = useI18n();
  const fileInputRef = useRef(null);
  const [seriesList, setSeriesList] = useState([]);
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [editingSeriesId, setEditingSeriesId] = useState(null);
  const [isDraggingOver, setIsDraggingOver] = useState(false);
  const [formData, setFormData] = useState({
    title: '',
    description: '',
    artwork: null
  });

  useEffect(() => {
    loadSeries();
  }, []);

  const loadSeries = async () => {
    const list = await getSermonSeries();
    setSeriesList(list);
  };

  const processFile = (file) => {
    if (file && file.type.startsWith('image/')) {
      const reader = new FileReader();
      reader.onloadend = () => {
        setFormData(prev => ({ ...prev, artwork: reader.result }));
      };
      reader.readAsDataURL(file);
    }
  };

  const handleImageUpload = (e) => {
    const file = e.target.files?.[0];
    if (file) processFile(file);
  };

  const handleDragOver = (e) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDraggingOver(true);
  };

  const handleDragLeave = (e) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDraggingOver(false);
  };

  const handleDrop = (e) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDraggingOver(false);
    const file = e.dataTransfer.files?.[0];
    if (file) processFile(file);
  };

  const openNewSeriesModal = () => {
    setEditingSeriesId(null);
    setFormData({ title: '', description: '', artwork: null });
    setIsDialogOpen(true);
  };

  const openEditSeriesModal = (series) => {
    setEditingSeriesId(series.id);
    setFormData({
      title: series.title || '',
      description: series.description || '',
      artwork: series.artwork || null
    });
    setIsDialogOpen(true);
  };

  const handleSave = async (e) => {
    e.preventDefault();
    if (!formData.title.trim()) return;

    const payload = {
      title: formData.title.trim(),
      description: formData.description,
      artwork: formData.artwork
    };

    if (editingSeriesId) {
      payload.id = editingSeriesId;
    }

    await saveSermonSeries(payload);

    setIsDialogOpen(false);
    setEditingSeriesId(null);
    setFormData({ title: '', description: '', artwork: null });
    loadSeries();
  };

  const handleDelete = async (id) => {
    if (window.confirm(t('confirmDeleteSeries'))) {
      await deleteSermonSeries(id);
      loadSeries();
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div>
          <Title1>{t('sermonSeriesTitle')}</Title1>
          <div style={{ color: tokens.colorNeutralForeground3, marginTop: '4px', fontSize: '14px' }}>
            {t('sermonSeriesDesc')}
          </div>
        </div>

        <Button appearance="primary" icon={<AddRegular />} onClick={openNewSeriesModal}>
          {t('newSeries')}
        </Button>

        <Dialog open={isDialogOpen} onOpenChange={(e, data) => {
          setIsDialogOpen(data.open);
          if (!data.open) {
            setEditingSeriesId(null);
            setFormData({ title: '', description: '', artwork: null });
          }
        }}>
          <DialogSurface>
            <form onSubmit={handleSave}>
              <DialogBody>
                <DialogTitle>{editingSeriesId ? t('editSeries') : t('createSermonSeries')}</DialogTitle>
                <DialogContent className={styles.form}>
                  <Field label={t('seriesTitle')} required>
                    <Input 
                      value={formData.title} 
                      onChange={e => setFormData({ ...formData, title: e.target.value })} 
                      placeholder="e.g. Grace Upon Grace, The Book of Romans..." 
                      required 
                    />
                  </Field>

                  <Field label={t('descriptionOptional')}>
                    <Textarea 
                      value={formData.description} 
                      onChange={(e, data) => setFormData({ ...formData, description: data.value })} 
                      placeholder="Theme, scripture references, or notes..." 
                    />
                  </Field>

                  <Field label={t('seriesArtworkGraphic')}>
                    <input 
                      type="file" 
                      accept="image/*" 
                      style={{ display: 'none' }} 
                      ref={fileInputRef}
                      onChange={handleImageUpload}
                    />
                    {!formData.artwork ? (
                      <div 
                        className={`${styles.dropzone} ${isDraggingOver ? styles.dropzoneActive : ''}`} 
                        onClick={() => fileInputRef.current?.click()}
                        onDragOver={handleDragOver}
                        onDragLeave={handleDragLeave}
                        onDrop={handleDrop}
                      >
                        <ImageRegular fontSize={32} />
                        <div>{t('dropzoneText')}</div>
                      </div>
                    ) : (
                      <div style={{ position: 'relative' }}>
                        <img src={formData.artwork} className={styles.previewImage} alt="Series Artwork Preview" />
                        <Button 
                          appearance="subtle" 
                          className={styles.deleteButton}
                          style={{ position: 'absolute', top: 8, right: 8, backgroundColor: 'rgba(0,0,0,0.6)' }} 
                          icon={<DeleteRegular />} 
                          onClick={() => setFormData({ ...formData, artwork: null })}
                        />
                      </div>
                    )}
                  </Field>
                </DialogContent>
                <DialogActions>
                  <DialogTrigger disableButtonEnhancement>
                    <Button appearance="secondary">{t('cancel')}</Button>
                  </DialogTrigger>
                  <Button type="submit" appearance="primary">{t('saveSeries')}</Button>
                </DialogActions>
              </DialogBody>
            </form>
          </DialogSurface>
        </Dialog>
      </div>

      <div className={styles.grid}>
        {seriesList.map(series => (
          <Card key={series.id} className={styles.card}>
            <div className={styles.cardBanner}>
              {series.artwork ? (
                <img src={series.artwork} alt={series.title} className={styles.cardBannerImage} />
              ) : (
                <div className={styles.placeholderArt}>
                  <ImageRegular fontSize={36} />
                  <span>{t('noArtworkUploaded')}</span>
                </div>
              )}
            </div>
            
            <div className={styles.cardContent}>
              <Title3 style={{ margin: 0, color: tokens.colorNeutralForeground1 }}>{series.title}</Title3>
              <div style={{ color: tokens.colorNeutralForeground3, fontSize: '13px', lineHeight: '1.4' }}>
                {series.description || '—'}
              </div>
            </div>

            <CardFooter style={{ justifyContent: 'space-between', borderTop: `1px solid ${tokens.colorNeutralStroke2}`, padding: '10px 16px', margin: 0 }}>
              <Button 
                appearance="subtle" 
                icon={<EditRegular />} 
                onClick={() => openEditSeriesModal(series)}
              >
                {t('editPlan')}
              </Button>
              <Button 
                appearance="subtle" 
                className={styles.deleteButton} 
                icon={<DeleteRegular />} 
                onClick={() => handleDelete(series.id)}
              >
                {t('delete')}
              </Button>
            </CardFooter>
          </Card>
        ))}
        {seriesList.length === 0 && (
          <div style={{ gridColumn: '1 / -1', textAlign: 'center', padding: '64px', color: tokens.colorNeutralForeground3 }}>
            {t('noSeriesFound')}
          </div>
        )}
      </div>
    </div>
  );
}
