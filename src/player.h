#ifndef PLAYER_H
#define PLAYER_H

#include "player_def.h"

void global_init();
CPlayer *player_create();
void player_destory(CPlayer *cp);

void packet_queue_init(PacketQueue *pkt_q);
int packet_queue_put(PacketQueue *pkt_q, AVPacket *pkt);
int packet_queue_get(PacketQueue *pkt_q, AVPacket *pkt, int block);
AudioState *stream_open(CPlayer *cp, const char *filename);
static void dump_metadata(void *ctx, AVDictionary *m, const char *indent);


/* control */
void cp_pause_audio();
void cp_stop_audio();
void cp_set_volume(int volume);
void cp_seek_audio(double percent);
void cp_seek_audio_by_sec(int sec);
void cp_seek_audio_by_absolute_pos(int pos);
int cp_get_time_length();
int cp_is_stopping();
double cp_get_current_time_pos();
void cp_free_player();
void cp_load_file(const char *filename);
#endif // PLAYER_H
