(function() {
  var MailCatcher,
    bind = function(fn, me){ return function(){ return fn.apply(me, arguments); }; };

  jQuery.expr.pseudos.icontains = function(a, i, m) {
    var ref, ref1;
    return ((ref = (ref1 = a.textContent) != null ? ref1 : a.innerText) != null ? ref : "").toUpperCase().indexOf(m[3].toUpperCase()) >= 0;
  };

  MailCatcher = (function() {
    function MailCatcher() {
      this.nextTab = bind(this.nextTab, this);
      this.previousTab = bind(this.previousTab, this);
      this.openTab = bind(this.openTab, this);
      this.selectedTab = bind(this.selectedTab, this);
      this.getTab = bind(this.getTab, this);
      $("#messages").on("click", "tr", (function(_this) {
        return function(e) {
          e.preventDefault();
          return _this.loadMessage($(e.currentTarget).attr("data-message-id"));
        };
      })(this));
      $("input[name=search]").on("keyup", (function(_this) {
        return function(e) {
          var query;
          query = $.trim($(e.currentTarget).val());
          if (query) {
            return _this.searchMessages(query);
          } else {
            return _this.clearSearch();
          }
        };
      })(this));
      $("#message").on("click", ".views .format.tab a", (function(_this) {
        return function(e) {
          e.preventDefault();
          return _this.loadMessageBody(_this.selectedMessage(), $($(e.currentTarget).parent("li")).data("message-format"));
        };
      })(this));
      $("#message iframe").on("load", (function(_this) {
        return function() {
          return _this.decorateMessageBody();
        };
      })(this));
      $("#resizer").on("mousedown", (function(_this) {
        return function(e) {
          var events;
          e.preventDefault();
          events = {
            mouseup: function(e) {
              e.preventDefault();
              return $(window).off(events);
            },
            mousemove: function(e) {
              e.preventDefault();
              return _this.resizeTo(e.clientY);
            }
          };
          return $(window).on(events);
        };
      })(this));
      this.resizeToSaved();
      $("nav.app .clear a").on("click", (function(_this) {
        return function(e) {
          e.preventDefault();
          if (confirm("You will lose all your received messages.\n\nAre you sure you want to clear all messages?")) {
            return $.ajax({
              url: new URL("messages", document.baseURI).toString(),
              type: "DELETE",
              success: function() {
                return _this.clearMessages();
              },
              error: function() {
                return alert("Error while clearing all messages.");
              }
            });
          }
        };
      })(this));
      $("nav.app .quit a").on("click", (function(_this) {
        return function(e) {
          e.preventDefault();
          if (confirm("You will lose all your received messages.\n\nAre you sure you want to quit?")) {
            _this.quitting = true;
            return $.ajax({
              type: "DELETE",
              success: function() {
                return _this.hasQuit();
              },
              error: function() {
                _this.quitting = false;
                return alert("Error while quitting.");
              }
            });
          }
        };
      })(this));
      this.favcount = new Favcount($("link[rel=\"icon\"]").attr("href"));
      key("up", (function(_this) {
        return function() {
          if (_this.selectedMessage()) {
            _this.loadMessage($("#messages tr.selected").prevAll(":visible").first().data("message-id"));
          } else {
            _this.loadMessage($("#messages tbody tr[data-message-id]").first().data("message-id"));
          }
          return false;
        };
      })(this));
      key("down", (function(_this) {
        return function() {
          if (_this.selectedMessage()) {
            _this.loadMessage($("#messages tr.selected").nextAll(":visible").data("message-id"));
          } else {
            _this.loadMessage($("#messages tbody tr[data-message-id]:first").data("message-id"));
          }
          return false;
        };
      })(this));
      key("⌘+up, ctrl+up", (function(_this) {
        return function() {
          _this.loadMessage($("#messages tbody tr[data-message-id]:visible").first().data("message-id"));
          return false;
        };
      })(this));
      key("⌘+down, ctrl+down", (function(_this) {
        return function() {
          _this.loadMessage($("#messages tbody tr[data-message-id]:visible").first().data("message-id"));
          return false;
        };
      })(this));
      key("left", (function(_this) {
        return function() {
          _this.openTab(_this.previousTab());
          return false;
        };
      })(this));
      key("right", (function(_this) {
        return function() {
          _this.openTab(_this.nextTab());
          return false;
        };
      })(this));
      key("backspace, delete", (function(_this) {
        return function() {
          var id;
          id = _this.selectedMessage();
          if (id != null) {
            $.ajax({
              url: new URL("messages/" + id, document.baseURI).toString(),
              type: "DELETE",
              success: function() {
                return _this.removeMessage(id);
              },
              error: function() {
                return alert("Error while removing message.");
              }
            });
          }
          return false;
        };
      })(this));
      this.refresh();
      this.subscribe();
    }

    MailCatcher.prototype.parseDateRegexp = /^(\d{4})[-\/\\](\d{2})[-\/\\](\d{2})(?:\s+|T)(\d{2})[:-](\d{2})[:-](\d{2})(?:([ +-]\d{2}:\d{2}|\s*\S+|Z?))?$/;

    MailCatcher.prototype.parseDate = function(date) {
      var match;
      if (match = this.parseDateRegexp.exec(date)) {
        return new Date(match[1], match[2] - 1, match[3], match[4], match[5], match[6], 0);
      }
    };

    MailCatcher.prototype.offsetTimeZone = function(date) {
      var offset;
      offset = Date.now().getTimezoneOffset() * 60000;
      date.setTime(date.getTime() - offset);
      return date;
    };

    MailCatcher.prototype.formatDate = function(date) {
      if (typeof date === "string") {
        date && (date = this.parseDate(date));
      }
      date && (date = this.offsetTimeZone(date));
      return date && (date = date.toString("dddd, d MMM yyyy h:mm:ss tt"));
    };

    MailCatcher.prototype.messagesCount = function() {
      return $("#messages tr").length - 1;
    };

    MailCatcher.prototype.updateMessagesCount = function() {
      this.favcount.set(this.messagesCount());
      return document.title = 'MailCatcher (' + this.messagesCount() + ')';
    };

    MailCatcher.prototype.tabs = function() {
      return $("#message ul").children(".tab");
    };

    MailCatcher.prototype.getTab = function(i) {
      return $(this.tabs()[i]);
    };

    MailCatcher.prototype.selectedTab = function() {
      return this.tabs().index($("#message li.tab.selected"));
    };

    MailCatcher.prototype.openTab = function(i) {
      return this.getTab(i).children("a").click();
    };

    MailCatcher.prototype.previousTab = function(tab) {
      var i;
      i = tab || tab === 0 ? tab : this.selectedTab() - 1;
      if (i < 0) {
        i = this.tabs().length - 1;
      }
      if (this.getTab(i).is(":visible")) {
        return i;
      } else {
        return this.previousTab(i - 1);
      }
    };

    MailCatcher.prototype.nextTab = function(tab) {
      var i;
      i = tab ? tab : this.selectedTab() + 1;
      if (i > this.tabs().length - 1) {
        i = 0;
      }
      if (this.getTab(i).is(":visible")) {
        return i;
      } else {
        return this.nextTab(i + 1);
      }
    };

    MailCatcher.prototype.haveMessage = function(message) {
      if (message.id != null) {
        message = message.id;
      }
      return $("#messages tbody tr[data-message-id=\"" + message + "\"]").length > 0;
    };

    MailCatcher.prototype.selectedMessage = function() {
      return $("#messages tr.selected").data("message-id");
    };

    MailCatcher.prototype.searchMessages = function(query) {
      var $rows, selector, token;
      selector = ((function() {
        var j, len, ref, results;
        ref = query.split(/\s+/);
        results = [];
        for (j = 0, len = ref.length; j < len; j++) {
          token = ref[j];
          results.push(":icontains('" + token + "')");
        }
        return results;
      })()).join("");
      $rows = $("#messages tbody tr");
      $rows.not(selector).hide();
      return $rows.filter(selector).show();
    };

    MailCatcher.prototype.clearSearch = function() {
      return $("#messages tbody tr").show();
    };

    MailCatcher.prototype.addMessage = function(message) {
      $("<tr />").attr("data-message-id", message.id.toString()).append($("<td/>").text(message.sender || "No sender").toggleClass("blank", !message.sender)).append($("<td/>").text((message.recipients || []).join(", ") || "No receipients").toggleClass("blank", !message.recipients.length)).append($("<td/>").text(message.subject || "No subject").toggleClass("blank", !message.subject)).append($("<td/>").text(this.formatDate(message.created_at))).prependTo($("#messages tbody"));
      return this.updateMessagesCount();
    };

    MailCatcher.prototype.removeMessage = function(id) {
      var isSelected, messageRow, switchTo;
      messageRow = $("#messages tbody tr[data-message-id=\"" + id + "\"]");
      isSelected = messageRow.is(".selected");
      if (isSelected) {
        switchTo = messageRow.next().data("message-id") || messageRow.prev().data("message-id");
      }
      messageRow.remove();
      if (isSelected) {
        if (switchTo) {
          this.loadMessage(switchTo);
        } else {
          this.unselectMessage();
        }
      }
      return this.updateMessagesCount();
    };

    MailCatcher.prototype.clearMessages = function() {
      $("#messages tbody tr").remove();
      this.unselectMessage();
      return this.updateMessagesCount();
    };

    MailCatcher.prototype.scrollToRow = function(row) {
      var overflow, relativePosition;
      relativePosition = row.offset().top - $("#messages").offset().top;
      if (relativePosition < 0) {
        return $("#messages").scrollTop($("#messages").scrollTop() + relativePosition - 20);
      } else {
        overflow = relativePosition + row.height() - $("#messages").height();
        if (overflow > 0) {
          return $("#messages").scrollTop($("#messages").scrollTop() + overflow + 20);
        }
      }
    };

    MailCatcher.prototype.unselectMessage = function() {
      $("#messages tbody, #message .metadata dd").empty();
      $("#message .metadata .attachments").hide();
      $("#message iframe").attr("src", "about:blank");
      return null;
    };

    MailCatcher.prototype.loadMessage = function(id) {
      var messageRow;
      if ((id != null ? id.id : void 0) != null) {
        id = id.id;
      }
      id || (id = $("#messages tr.selected").attr("data-message-id"));
      if (id != null) {
        $("#messages tbody tr:not([data-message-id='" + id + "'])").removeClass("selected");
        messageRow = $("#messages tbody tr[data-message-id='" + id + "']");
        messageRow.addClass("selected");
        this.scrollToRow(messageRow);
        return $.getJSON("messages/" + id + ".json", (function(_this) {
          return function(message) {
            var $ul;
            $("#message .metadata dd.created_at").text(_this.formatDate(message.created_at));
            $("#message .metadata dd.from").text(message.sender);
            $("#message .metadata dd.to").text((message.recipients || []).join(", "));
            $("#message .metadata dd.subject").text(message.subject);
            $("#message .views .tab.format").each(function(i, el) {
              var $el, format;
              $el = $(el);
              format = $el.attr("data-message-format");
              if ($.inArray(format, message.formats) >= 0) {
                $el.find("a").attr("href", "messages/" + id + "." + format);
                return $el.show();
              } else {
                return $el.hide();
              }
            });
            if ($("#message .views .tab.selected:not(:visible)").length) {
              $("#message .views .tab.selected").removeClass("selected");
              $("#message .views .tab:visible:first").addClass("selected");
            }
            if (message.attachments.length) {
              $ul = $("<ul/>").appendTo($("#message .metadata dd.attachments").empty());
              $.each(message.attachments, function(i, attachment) {
                return $ul.append($("<li>").append($("<a>").attr("href", "messages/" + id + "/parts/" + attachment["cid"]).addClass(attachment["type"].split("/", 1)[0]).addClass(attachment["type"].replace("/", "-")).text(attachment["filename"])));
              });
              $("#message .metadata .attachments").show();
            } else {
              $("#message .metadata .attachments").hide();
            }
            $("#message .views .download a").attr("href", "messages/" + id + ".eml");
            return _this.loadMessageBody();
          };
        })(this));
      }
    };

    MailCatcher.prototype.loadMessageBody = function(id, format) {
      id || (id = this.selectedMessage());
      format || (format = $("#message .views .tab.format.selected").attr("data-message-format"));
      format || (format = "html");
      $("#message .views .tab[data-message-format=\"" + format + "\"]:not(.selected)").addClass("selected");
      $("#message .views .tab:not([data-message-format=\"" + format + "\"]).selected").removeClass("selected");
      if (id != null) {
        return $("#message iframe").attr("src", "messages/" + id + "." + format);
      }
    };

    MailCatcher.prototype.decorateMessageBody = function() {
      var body, format, message_iframe, text;
      format = $("#message .views .tab.format.selected").attr("data-message-format");
      switch (format) {
        case "html":
          body = $("#message iframe").contents().find("body");
          return $("a", body).attr("target", "_blank");
        case "plain":
          message_iframe = $("#message iframe").contents();
          text = message_iframe.text();
          text = text.replace(/&/g, "&amp;");
          text = text.replace(/</g, "&lt;");
          text = text.replace(/>/g, "&gt;");
          text = text.replace(/"/g, "&quot;");
          text = text.replace(/((http|ftp|https):\/\/[\w\-_]+(\.[\w\-_]+)+([\w\-\.,@?^=%&amp;:\/~\+#]*[\w\-\@?^=%&amp;\/~\+#])?)/g, "<a href=\"$1\" target=\"_blank\">$1</a>");
          return message_iframe.find("html").html("<body style=\"font-family: sans-serif; white-space: pre-wrap\">" + text + "</body>");
      }
    };

    MailCatcher.prototype.refresh = function() {
      return $.getJSON("/api/messages", (function(_this) {
        return function(messages) {
          $.each(messages, function(i, message) {
            if (!_this.haveMessage(message)) {
              return _this.addMessage(message);
            }
          });
          return _this.updateMessagesCount();
        };
      })(this));
    };

    MailCatcher.prototype.subscribe = function() {
      if (typeof WebSocket !== "undefined" && WebSocket !== null) {
        // TODO: Websocket here
        // return this.subscribeWebSocket();
      } else {
        return this.subscribePoll();
      }
    };

    MailCatcher.prototype.subscribeWebSocket = function() {
      var secure, url;
      secure = window.location.protocol === "https:";
      url = new URL("messages", document.baseURI);
      url.protocol = secure ? "wss" : "ws";
      this.websocket = new WebSocket(url.toString());
      return this.websocket.onmessage = (function(_this) {
        return function(event) {
          var data;
          data = JSON.parse(event.data);
          if (data.type === "add") {
            return _this.addMessage(data.message);
          } else if (data.type === "remove") {
            return _this.removeMessage(data.id);
          } else if (data.type === "clear") {
            return _this.clearMessages();
          } else if (data.type === "quit" && !_this.quitting) {
            alert("MailCatcher has been quit");
            return _this.hasQuit();
          }
        };
      })(this);
    };

    MailCatcher.prototype.subscribePoll = function() {
      if (this.refreshInterval == null) {
        return this.refreshInterval = setInterval(((function(_this) {
          return function() {
            return _this.refresh();
          };
        })(this)), 1000);
      }
    };

    MailCatcher.prototype.resizeToSavedKey = "mailcatcherSeparatorHeight";

    MailCatcher.prototype.resizeTo = function(height) {
      var ref;
      $("#messages").css({
        height: height - $("#messages").offset().top
      });
      return (ref = window.localStorage) != null ? ref.setItem(this.resizeToSavedKey, height) : void 0;
    };

    MailCatcher.prototype.resizeToSaved = function() {
      var height, ref;
      height = parseInt((ref = window.localStorage) != null ? ref.getItem(this.resizeToSavedKey) : void 0);
      if (!isNaN(height)) {
        return this.resizeTo(height);
      }
    };

    MailCatcher.prototype.hasQuit = function() {
      return location.assign($("body > header h1 a").attr("href"));
    };

    return MailCatcher;

  })();

  $(function() {
    return window.MailCatcher = new MailCatcher;
  });

}).call(this);
