# Manda una notifica su ntfy (https://ntfy.sh). Serve `curl` e `jq`.
# Uso: notifica "titolo" "messaggio" priorità(1-5) tag
#
# Il topic sta in notify.conf, nella stessa cartella, su una riga:
#   NTFY_TOPIC=un-nome-lungo-e-casuale
# Chi conosce il topic può leggere e mandare notifiche: scegli un nome lungo e casuale,
# tieni il file a permessi 600 e non metterlo mai su git. Senza notify.conf non succede niente.
[ -f "$DIR/notify.conf" ] && . "$DIR/notify.conf"
notifica() {
  [ -n "$NTFY_TOPIC" ] || return 0
  jq -n --arg t "$NTFY_TOPIC" --arg ti "$1" --arg m "$2" --argjson p "$3" --arg tag "$4" \
    '{topic:$t, title:$ti, message:$m, priority:$p, tags:[$tag]}' |
    curl -s --max-time 20 -o /dev/null -w "ntfy: HTTP %{http_code}\n" \
      -H "Content-Type: application/json" -d @- https://ntfy.sh
}
