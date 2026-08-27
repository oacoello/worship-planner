import React, { useState, useEffect } from 'react';
import {
  Title1,
  Title3,
  Button,
  Card,
  CardHeader,
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
  Select,
  Accordion,
  AccordionItem,
  AccordionHeader,
  AccordionPanel,
  Badge
} from '@fluentui/react-components';
import { AddRegular, DeleteRegular, MusicNote2Regular, SearchRegular } from '@fluentui/react-icons';
import { getSongs, saveSong, deleteSong } from '../db';
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
    marginBottom: '24px',
  },
  controlsRow: {
    display: 'flex',
    gap: '16px',
    marginBottom: '24px',
  },
  searchInput: {
    flex: 1,
    maxWidth: '400px',
  },
  list: {
    display: 'flex',
    flexDirection: 'column',
    gap: '12px',
  },
  songCard: {
    backgroundColor: tokens.colorNeutralBackground1,
    borderTop: `1px solid ${tokens.colorNeutralStroke1}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke1}`,
    borderRadius: tokens.borderRadiusMedium,
    overflow: 'hidden',
  },
  arrangementsSection: {
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    marginTop: '12px',
    padding: '12px',
    backgroundColor: tokens.colorNeutralBackground2,
    borderRadius: tokens.borderRadiusMedium,
  },
  arrangementRow: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    padding: '8px 12px',
    backgroundColor: tokens.colorNeutralBackground1,
    borderRadius: tokens.borderRadiusSmall,
    borderTop: `1px solid ${tokens.colorNeutralStroke2}`,
    borderBottom: `1px solid ${tokens.colorNeutralStroke2}`,
    borderLeft: `1px solid ${tokens.colorNeutralStroke2}`,
    borderRight: `1px solid ${tokens.colorNeutralStroke2}`,
  },
  form: {
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
    marginTop: '16px'
  },
  deleteButton: {
    color: tokens.colorPaletteRedForeground1,
    '&:hover': {
      color: tokens.colorPaletteRedForeground2,
      backgroundColor: tokens.colorPaletteRedBackground2
    }
  }
});

const DEFAULT_KEYS = ['C', 'Db', 'D', 'Eb', 'E', 'F', 'F#', 'G', 'Ab', 'A', 'Bb', 'B', 'Am', 'Bm', 'Cm', 'Dm', 'Em', 'F#m', 'Gm'];

export default function SongBank() {
  const styles = useStyles();
  const { t } = useI18n();
  const [songs, setSongs] = useState([]);
  const [searchQuery, setSearchQuery] = useState('');
  
  // Dialog state for adding a whole song
  const [isSongDialogOpen, setIsSongDialogOpen] = useState(false);
  const [newSongData, setNewSongData] = useState({
    title: '',
    artist: '',
    defaultArrangementName: 'Standard',
    key: 'G',
    bpm: 72,
    durationMins: 4,
    durationSecs: 30
  });

  // Dialog state for adding an arrangement to an existing song
  const [isArrDialogOpen, setIsArrDialogOpen] = useState(false);
  const [targetSong, setTargetSong] = useState(null);
  const [newArrData, setNewArrData] = useState({
    name: 'Acoustic',
    key: 'G',
    bpm: 72,
    durationMins: 4,
    durationSecs: 0
  });

  useEffect(() => {
    loadSongs();
  }, []);

  const loadSongs = async () => {
    const list = await getSongs();
    setSongs(list);
  };

  const handleCreateSong = async (e) => {
    e.preventDefault();
    if (!newSongData.title.trim()) return;

    const initialArrangement = {
      id: crypto.randomUUID ? crypto.randomUUID() : 'arr_' + Date.now(),
      name: newSongData.defaultArrangementName.trim() || 'Standard',
      key: newSongData.key,
      bpm: parseInt(newSongData.bpm) || 0,
      duration: (parseInt(newSongData.durationMins) || 0) * 60 + (parseInt(newSongData.durationSecs) || 0)
    };

    await saveSong({
      title: newSongData.title.trim(),
      artist: newSongData.artist.trim(),
      arrangements: [initialArrangement]
    });

    setIsSongDialogOpen(false);
    setNewSongData({
      title: '',
      artist: '',
      defaultArrangementName: 'Standard',
      key: 'G',
      bpm: 72,
      durationMins: 4,
      durationSecs: 30
    });
    loadSongs();
  };

  const handleAddArrangement = async (e) => {
    e.preventDefault();
    if (!targetSong || !newArrData.name.trim()) return;

    const arrangement = {
      id: crypto.randomUUID ? crypto.randomUUID() : 'arr_' + Date.now(),
      name: newArrData.name.trim(),
      key: newArrData.key,
      bpm: parseInt(newArrData.bpm) || 0,
      duration: (parseInt(newArrData.durationMins) || 0) * 60 + (parseInt(newArrData.durationSecs) || 0)
    };

    const updatedSong = {
      ...targetSong,
      arrangements: [...(targetSong.arrangements || []), arrangement]
    };

    await saveSong(updatedSong);
    setIsArrDialogOpen(false);
    setTargetSong(null);
    loadSongs();
  };

  const handleDeleteSong = async (id) => {
    if (window.confirm(t('confirmDeleteSong'))) {
      await deleteSong(id);
      loadSongs();
    }
  };

  const handleDeleteArrangement = async (song, arrId) => {
    if ((song.arrangements || []).length <= 1) {
      alert('A song must have at least one arrangement.');
      return;
    }
    const updated = {
      ...song,
      arrangements: song.arrangements.filter(a => a.id !== arrId)
    };
    await saveSong(updated);
    loadSongs();
  };

  const filteredSongs = songs.filter(s => 
    s.title.toLowerCase().includes(searchQuery.toLowerCase()) || 
    (s.artist && s.artist.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div>
          <Title1>{t('songBankTitle')}</Title1>
          <div style={{ color: tokens.colorNeutralForeground3, marginTop: '4px', fontSize: '14px' }}>
            {t('songBankDesc')}
          </div>
        </div>

        <Dialog open={isSongDialogOpen} onOpenChange={(e, data) => setIsSongDialogOpen(data.open)}>
          <DialogTrigger disableButtonEnhancement>
            <Button appearance="primary" icon={<AddRegular />}>{t('newSong')}</Button>
          </DialogTrigger>
          <DialogSurface>
            <form onSubmit={handleCreateSong}>
              <DialogBody>
                <DialogTitle>{t('newSong')}</DialogTitle>
                <DialogContent className={styles.form}>
                  <Field label={t('songTitle')} required>
                    <Input 
                      value={newSongData.title} 
                      onChange={e => setNewSongData({ ...newSongData, title: e.target.value })} 
                      placeholder="e.g. Way Maker, Living Hope..." 
                      required 
                    />
                  </Field>

                  <Field label={t('artistAuthor')}>
                    <Input 
                      value={newSongData.artist} 
                      onChange={e => setNewSongData({ ...newSongData, artist: e.target.value })} 
                      placeholder="e.g. Sinach, Phil Wickham..." 
                    />
                  </Field>

                  <div style={{ borderTop: `1px solid ${tokens.colorNeutralStroke1}`, borderBottom: `1px solid ${tokens.colorNeutralStroke1}`, borderLeft: `1px solid ${tokens.colorNeutralStroke1}`, borderRight: `1px solid ${tokens.colorNeutralStroke1}`, padding: '16px', borderRadius: tokens.borderRadiusMedium, backgroundColor: tokens.colorNeutralBackground2 }}>
                    <div style={{ fontWeight: '600', marginBottom: '12px', fontSize: '13px' }}>{t('defaultArrangementName')}</div>
                    
                    <Field label={t('arrangementName')} required style={{ marginBottom: '12px' }}>
                      <Input 
                        value={newSongData.defaultArrangementName} 
                        onChange={e => setNewSongData({ ...newSongData, defaultArrangementName: e.target.value })} 
                        placeholder="e.g. Standard, Full Band, Acoustic..." 
                        required 
                      />
                    </Field>

                    <div style={{ display: 'flex', gap: '16px', marginBottom: '12px' }}>
                      <Field label={t('key')} style={{ flex: 1 }}>
                        <Select 
                          value={newSongData.key} 
                          onChange={e => setNewSongData({ ...newSongData, key: e.target.value })}
                        >
                          {DEFAULT_KEYS.map(k => (
                            <option key={k} value={k}>{k}</option>
                          ))}
                        </Select>
                      </Field>
                      <Field label={t('bpm')} style={{ flex: 1 }}>
                        <Input 
                          type="number" 
                          value={newSongData.bpm} 
                          onChange={e => setNewSongData({ ...newSongData, bpm: e.target.value })} 
                        />
                      </Field>
                    </div>

                    <div style={{ display: 'flex', gap: '16px' }}>
                      <Field label={t('durationMinutes')} style={{ flex: 1 }}>
                        <Input 
                          type="number" 
                          value={newSongData.durationMins} 
                          onChange={e => setNewSongData({ ...newSongData, durationMins: e.target.value })} 
                        />
                      </Field>
                      <Field label={t('durationSeconds')} style={{ flex: 1 }}>
                        <Input 
                          type="number" 
                          max="59"
                          value={newSongData.durationSecs} 
                          onChange={e => setNewSongData({ ...newSongData, durationSecs: e.target.value })} 
                        />
                      </Field>
                    </div>
                  </div>
                </DialogContent>
                <DialogActions>
                  <DialogTrigger disableButtonEnhancement>
                    <Button appearance="secondary">{t('cancel')}</Button>
                  </DialogTrigger>
                  <Button type="submit" appearance="primary">{t('addSongToBank')}</Button>
                </DialogActions>
              </DialogBody>
            </form>
          </DialogSurface>
        </Dialog>
      </div>

      <div className={styles.controlsRow}>
        <Input 
          className={styles.searchInput}
          contentBefore={<SearchRegular />}
          placeholder={t('searchSongs')}
          value={searchQuery}
          onChange={e => setSearchQuery(e.target.value)}
        />
      </div>

      {/* Add Arrangement Dialog */}
      <Dialog open={isArrDialogOpen} onOpenChange={(e, data) => setIsArrDialogOpen(data.open)}>
        <DialogSurface>
          <form onSubmit={handleAddArrangement}>
            <DialogBody>
              <DialogTitle>{t('addArrangement')}: {targetSong?.title}</DialogTitle>
              <DialogContent className={styles.form}>
                <Field label={t('arrangementName')} required>
                  <Input 
                    value={newArrData.name} 
                    onChange={e => setNewArrData({ ...newArrData, name: e.target.value })} 
                    placeholder="e.g. Acoustic, Live 2026, Radio Edit..." 
                    required 
                  />
                </Field>

                <div style={{ display: 'flex', gap: '16px' }}>
                  <Field label={t('key')} style={{ flex: 1 }}>
                    <Select 
                      value={newArrData.key} 
                      onChange={e => setNewArrData({ ...newArrData, key: e.target.value })}
                    >
                      {DEFAULT_KEYS.map(k => (
                        <option key={k} value={k}>{k}</option>
                      ))}
                    </Select>
                  </Field>
                  <Field label={t('bpm')} style={{ flex: 1 }}>
                    <Input 
                      type="number" 
                      value={newArrData.bpm} 
                      onChange={e => setNewArrData({ ...newArrData, bpm: e.target.value })} 
                    />
                  </Field>
                </div>

                <div style={{ display: 'flex', gap: '16px' }}>
                  <Field label={t('durationMinutes')} style={{ flex: 1 }}>
                    <Input 
                      type="number" 
                      value={newArrData.durationMins} 
                      onChange={e => setNewArrData({ ...newArrData, durationMins: e.target.value })} 
                    />
                  </Field>
                  <Field label={t('durationSeconds')} style={{ flex: 1 }}>
                    <Input 
                      type="number" 
                      max="59"
                      value={newArrData.durationSecs} 
                      onChange={e => setNewArrData({ ...newArrData, durationSecs: e.target.value })} 
                    />
                  </Field>
                </div>
              </DialogContent>
              <DialogActions>
                <DialogTrigger disableButtonEnhancement>
                  <Button appearance="secondary">{t('cancel')}</Button>
                </DialogTrigger>
                <Button type="submit" appearance="primary">{t('addArrangement')}</Button>
              </DialogActions>
            </DialogBody>
          </form>
        </DialogSurface>
      </Dialog>

      {/* Compact Table Header */}
      <div style={{
        display: 'grid',
        gridTemplateColumns: '1fr 80px 100px 100px 150px 100px',
        padding: '8px 16px',
        borderBottom: `1px solid ${tokens.colorNeutralStroke1}`,
        color: tokens.colorNeutralForeground3,
        fontSize: '12px',
        fontWeight: '600',
        letterSpacing: '0.5px',
        marginTop: '16px'
      }}>
        <div>TITLE</div>
        <div style={{ textAlign: 'center' }}>BPM</div>
        <div style={{ textAlign: 'center' }}>KEYS</div>
        <div style={{ textAlign: 'center' }}>DURATION</div>
        <div style={{ textAlign: 'center' }}>ARRANGEMENTS</div>
        <div style={{ textAlign: 'right' }}>ACTIONS</div>
      </div>

      <div className={styles.list}>
        {filteredSongs.map(song => {
          const arrCount = (song.arrangements || []).length;
          // Aggregate keys and bpms to show a quick summary
          const allKeys = [...new Set((song.arrangements || []).map(a => a.key).filter(Boolean))];
          const allBpms = [...new Set((song.arrangements || []).map(a => a.bpm).filter(b => b > 0))];
          const keySummary = allKeys.length > 0 ? allKeys.join(', ') : '-';
          const bpmSummary = allBpms.length > 0 
            ? (allBpms.length === 1 ? allBpms[0] : `${Math.min(...allBpms)}-${Math.max(...allBpms)}`)
            : '-';
          
          return (
            <Accordion key={song.id} collapsible style={{ 
              backgroundColor: tokens.colorNeutralBackground1,
              borderBottom: `1px solid ${tokens.colorNeutralStroke2}`
            }}>
              <AccordionItem value="1">
                <AccordionHeader style={{ margin: 0, padding: '4px 0' }} size="small">
                  <div style={{
                    display: 'grid',
                    gridTemplateColumns: '1fr 80px 100px 100px 150px 100px',
                    width: '100%',
                    alignItems: 'center',
                    fontSize: '13px'
                  }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                      <MusicNote2Regular fontSize={16} style={{ color: tokens.colorNeutralForeground3 }} />
                      <div>
                        <div style={{ fontWeight: '600', color: tokens.colorNeutralForeground1 }}>{song.title}</div>
                        {song.artist && <div style={{ fontSize: '11px', color: tokens.colorNeutralForeground3 }}>{song.artist}</div>}
                      </div>
                    </div>
                    <div style={{ textAlign: 'center', color: tokens.colorNeutralForeground2 }}>{bpmSummary}</div>
                    <div style={{ textAlign: 'center', color: tokens.colorNeutralForeground2 }}>{keySummary}</div>
                    <div style={{ textAlign: 'center', color: tokens.colorNeutralForeground2 }}>-</div>
                    <div style={{ textAlign: 'center', color: tokens.colorNeutralForeground3 }}>{arrCount} arr.</div>
                    
                    {/* Actions block propagation to prevent accordion toggle when clicking buttons */}
                    <div style={{ textAlign: 'right', display: 'flex', gap: '4px', justifyContent: 'flex-end' }} onClick={e => e.stopPropagation()}>
                      <Button 
                        size="small" 
                        appearance="subtle"
                        icon={<AddRegular fontSize={14} />} 
                        onClick={() => {
                          setTargetSong(song);
                          setIsArrDialogOpen(true);
                        }}
                        title={t('addArrangement')}
                      />
                      <Button 
                        size="small" 
                        appearance="subtle" 
                        className={styles.deleteButton} 
                        icon={<DeleteRegular fontSize={14} />} 
                        onClick={() => handleDeleteSong(song.id)}
                        title={t('delete')}
                      />
                    </div>
                  </div>
                </AccordionHeader>
                
                <AccordionPanel style={{ padding: '0 0 12px 32px' }}>
                  <div style={{
                    display: 'flex',
                    flexDirection: 'column',
                    backgroundColor: tokens.colorNeutralBackground2,
                    borderRadius: tokens.borderRadiusSmall,
                    overflow: 'hidden',
                    border: `1px solid ${tokens.colorNeutralStroke3}`
                  }}>
                    {(song.arrangements || []).map((arr, idx) => (
                      <div key={arr.id} style={{
                        display: 'grid',
                        gridTemplateColumns: '1fr 80px 100px 100px 150px 100px',
                        padding: '6px 16px',
                        alignItems: 'center',
                        fontSize: '12px',
                        borderBottom: idx < (song.arrangements || []).length - 1 ? `1px solid ${tokens.colorNeutralStroke2}` : 'none'
                      }}>
                        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                          <span style={{ fontWeight: '500' }}>{arr.name}</span>
                        </div>
                        <div style={{ textAlign: 'center', color: tokens.colorNeutralForeground2 }}>{arr.bpm > 0 ? arr.bpm : '-'}</div>
                        <div style={{ textAlign: 'center' }}>
                          <Badge size="small" appearance="tint" color="brand">{arr.key}</Badge>
                        </div>
                        <div style={{ textAlign: 'center', fontFamily: 'monospace', color: tokens.colorNeutralForeground2 }}>
                          {Math.floor((arr.duration || 0) / 60)}:{((arr.duration || 0) % 60).toString().padStart(2, '0')}
                        </div>
                        <div></div>
                        <div style={{ textAlign: 'right' }}>
                          {(song.arrangements || []).length > 1 && (
                            <Button 
                              appearance="subtle" 
                              size="small" 
                              className={styles.deleteButton} 
                              icon={<DeleteRegular fontSize={12} />} 
                              onClick={() => handleDeleteArrangement(song, arr.id)} 
                            />
                          )}
                        </div>
                      </div>
                    ))}
                  </div>
                </AccordionPanel>
              </AccordionItem>
            </Accordion>
          );
        })}

        {filteredSongs.length === 0 && (
          <div style={{ textAlign: 'center', padding: '64px', color: tokens.colorNeutralForeground3 }}>
            {t('noSongsFound')}
          </div>
        )}
      </div>
    </div>
  );
}
